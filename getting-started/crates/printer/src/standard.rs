/// Printer 的一种实现

use std::cell::{Cell, RefCell};
use std::{cmp, io};
use std::io::Write;
use termcolor::WriteColor;
use grep_matcher::{LineTerminator, Match, Matcher};
use crate::color::ColorSpecs;
use crate::counter::CounterWriter;
use crate::util::trim_ascii_prefix;

#[derive(Debug, Clone)]
struct Config {
    /// 各种内容的输出颜色定制类型
    colors: ColorSpecs,
    /// 是否裁剪掉 ascii 空白字符
    trim_ascii: bool,
    /// 单行的长度最大值
    max_columns: Option<u64>,

}

impl Default for Config {
    fn default() -> Self {
        Config {
            colors: ColorSpecs::default(),
            trim_ascii: false,
            max_columns: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StandardBuilder {
    config: Config,
}

impl StandardBuilder {
    pub fn new() -> StandardBuilder {
        StandardBuilder { config: Config::default() }
    }

    pub fn build<W: WriteColor>(&self, wtr: W) -> Standard<W> {
        Standard {
            config: self.config.clone(),
            wtr: RefCell::new(CounterWriter::new(wtr)),
            matches: vec![],
        }
    }

    pub fn color_specs(&mut self, specs: ColorSpecs) -> &mut StandardBuilder {
        self.config.colors = specs;
        self
    }

    pub fn trim_ascii(&mut self, yes: bool) -> &mut StandardBuilder {
        self.config.trim_ascii = yes;
        self
    }

    pub fn max_columns(&mut self, limit: Option<u64>) -> &mut StandardBuilder {
        self.config.max_columns = limit;
        self
    }
}

/// 标准输出的Printer类型
#[derive(Clone, Debug)]
pub struct Standard<W> {
    /// Standard Printer 的配置
    config: Config,
    /// W 是 std::io::Write 类型， wtr 是 writer 的缩写， 即输出目的地
    wtr: RefCell<CounterWriter<W>>,
    /// 使用向量存储匹配的字符串在行中的位置范围，因为一行中可能有多个匹配的字符串所以用向量存储
    matches: Vec<Match>,
}

impl<W: WriteColor> Standard<W> {

    pub fn sink<'s>(&'s mut self) -> StandardSink<'s, W> {
        StandardSink {
            standard: self,
        }
    }
}

///
#[derive(Debug)]
// struct StandardImpl<'a, M: Matcher, W> {
struct StandardImpl<'a, W> {
    // searcher: &'a Searcher,
    // sink: &'a StandardSink<'a, 'a, M, W>,
    sink: &'a StandardSink<'a, W>,
    // sunk: Sunk<'a>,
    /// 是否已经为匹配的字段设置好颜色显示，当输出无颜色的字符串前清除（false），当输出有颜色的字符串前设置（true）
    in_color_match: Cell<bool>,
}

// impl<'a, M: Matcher, W: WriteColor> StandardImpl<'a, M, W> {
impl<'a, W: WriteColor> StandardImpl<'a, W> {

    fn new(
        // searcher: &'a Searcher,
        sink: &'a StandardSink<'_, W>,
    ) -> StandardImpl<'a, W> {
        StandardImpl {
            // searcher,
            sink,
            // sunk: Sunk::empty(),
            in_color_match: Cell::new(false),
        }
    }

    /// 根据配置决定是否使用颜色高亮输出匹配的行
    fn write_colored_line(
        &self,
        matches: &[Match],      //bytes中匹配的切片的range范围，可能有多个
        bytes: &[u8],           //推测是匹配的字符串，里面可能包含多行文本
    ) -> io::Result<()> {
        let spec = self.config().colors.matched();
        if !self.wtr().borrow().supports_color() || spec.is_none() { //如果 WriteColor 不支持颜色输出、或者没有设置颜色
            return self.write_line(bytes);
        }

        let mut line = Match::new(0, bytes.len());
        self.trim_ascii_prefix(bytes, &mut line);
        if self.exceeds_max_columns(bytes) {
            // self.write_exceeded_line(bytes, line, matches, &mut 0)
            self.write(b"[Omitted long context line]")
        } else {
            self.write_colored_matches(bytes, line, matches, &mut 0)?;
            self.write_line_term()?;
            Ok(())
        }
    }

    fn config(&self) -> &'a Config {
        &self.sink.standard.config
    }

    /// 无颜色输出匹配的行
    fn write_line(&self, line: &[u8]) -> io::Result<()> {
        /// 裁剪 line 中前缀 ascii 空白字符操作
        let line = if !self.config().trim_ascii {
            line
        } else {
            // let line_terminator = self.searcher.line_terminator();   //TODO
            let line_terminator = LineTerminator::default();
            let full_range = Match::new(0, line.len());
            let range = trim_ascii_prefix(line_terminator, line, full_range);
            &line[range]
        };

        if self.exceeds_max_columns(line) { //当前行大于设置的行最大长度
            // let range = Match::new(0, line.len());
            // ripgrep 对于超过最大长度的行的处理参考 write_exceeded_line(） 这个方法，不是很重要的功能这里不深入了
            // self.write_exceeded_line(
            //     line,
            //     range,
            //     self.sunk.matches(),
            //     &mut 0,
            // )?;
            self.write(b"[Omitted long context line]")?;
        } else {
            self.write(line)?;  //直接输出行
            if !self.has_line_terminator(line) {    //没有行终止符就输出一个行终止符进行换行
                self.write_line_term()?;
            }
        }
        Ok(())
    }

    fn exceeds_max_columns(&self, line: &[u8]) -> bool {
        // max_columns 为空 None, 就返回默认值 false, 否则调用闭包判断当前行的长度是否大于设置的行最大长度
        self.config().max_columns.map_or(false, |m| line.len() as u64 > m)
    }

    fn has_line_terminator(&self, buf: &[u8]) -> bool {
        // TODO: 先用默认的行终止符替换调通基本功能
        // self.searcher.line_terminator().is_suffix(buf)
        LineTerminator::default().is_suffix(buf)
    }

    /// 输出匹配的行
    /// 匹配的行的输出是一部分一部分写的，因为termcolor对颜色的控制无法做到精细控制，
    /// 只能写完非高亮部分，然后设置颜色再写颜色高亮部分，写完高亮部分再重置颜色配置再写非高亮部分，这样交替
    fn write_colored_matches(&self,
                             bytes: &[u8],          //推测是匹配的字符串，里面可能包含多行文本
                             mut line: Match,       //匹配的行的range范围
                             matches: &[Match],     //bytes中匹配的切片的range范围，可能有多个
                             match_index: &mut usize,   //matches 的索引，初始为 0 (即 *match_index == 0)
    ) -> io::Result<()> {
        //裁剪匹配行中的行终止符，TODO

        if matches.is_empty() {     //行整体匹配但是里面没有匹配的切片
            self.write(&bytes[line])?;  // container[index] 实际上是容器的语法糖， container 需要实现 std::ops::Index 特征
            return Ok(());
        }
        // 处理行中匹配的切片，红色高亮显示
        // 比如 “This is free and unencumbered software released into the public domain.” 正则匹配的是 "domain",
        // 打印这行时要高亮展示“domain”
        while !line.is_empty() {
            if matches[*match_index].end() <= line.start() {
                if *match_index + 1 < matches.len() {
                    *match_index += 1;
                    continue;
                } else {
                    self.end_color_match()?;    //即这行没有需要颜色高亮展示匹配切片字符串，重置 WriteColor 对象的配置
                    self.write(&bytes[line])?;
                    break;
                }
            }
            // 交替写非高亮部分和高亮部分
            let m = matches[*match_index];
            if line.start() < m.start() {   //非高亮部分
                let upto = cmp::min(line.end(), m.start());
                self.end_color_match()?;
                self.write(&bytes[line.with_end(upto)])?;
                line = line.with_start(upto);
            } else {    //高亮部分
                let upto = cmp::min(line.end(), m.end());
                self.start_color_match()?;
                self.write(&bytes[line.with_end(upto)])?;
                line = line.with_start(upto);
            }
        }
        self.end_color_match()?;
        Ok(())
    }

    fn write_line_term(&self) -> io::Result<()> {
        // TODO
        // self.write(self.searcher.line_terminator().as_bytes())
        self.write(LineTerminator::default().as_bytes())
    }

    /// 将数据写入 io::Write 即输出
    fn write(&self, buf: &[u8]) -> io::Result<()> {
        self.wtr().borrow_mut().write_all(buf)
    }

    fn wtr(&self) -> &'a RefCell<CounterWriter<W>> {
        &self.sink.standard.wtr
    }

    /// 设置 WriteColor Writer 的颜色配置
    /// 写匹配的行的高亮部分时使用
    fn start_color_match(&self) -> io::Result<()> {
        if self.in_color_match.get() {
            return Ok(());
        }
        self.wtr().borrow_mut().set_color(self.config().colors.matched())?;
        self.in_color_match.set(true);
        Ok(())
    }

    /// 重置 WriteColor Writer 的配置，比如清除颜色配置
    /// 写匹配的行的非高亮部分时使用
    fn end_color_match(&self) -> io::Result<()> {
        if !self.in_color_match.get() {
            return Ok(());
        }
        self.wtr().borrow_mut().reset()?;
        self.in_color_match.set(false);
        Ok(())
    }

    /// 裁剪前缀空白字符（包括制表符、换行符、空格等）
    fn trim_ascii_prefix(&self, slice: &[u8], range: &mut Match) {
        //TODO
    }
}

/// 对
#[derive(Debug)]
// pub struct StandardSink<'p, 's, M: Matcher, W> {
pub struct StandardSink<'s, W> {
    // matcher: M,
    standard: &'s mut Standard<W>,
    // replacer: Replacer<M>,
    // interpolator: hyperlink::Interpolator,
    // path: Option<PrinterPath<'p>>,
    // start_time: Instant,
    // match_count: u64,
    // after_context_remaining: u64,
    // binary_byte_offset: Option<u64>,
    // stats: Option<Stats>,
    // needs_match_granularity: bool,
}

// impl<'p, 's, M: Matcher, W: WriteColor> StandardSink<'p, 's, M, W> {
impl<'s, W: WriteColor> StandardSink<'s, W> {

}

#[cfg(test)]
mod tests {
    use termcolor::ColorChoice;
    use grep_matcher::Match;
    use crate::standard::{StandardBuilder, StandardImpl};

    // 参考 hiargs.rs 中从 printer() 创建 Printer 实例到 ReadByLine run() 中输出匹配结果的流程
    #[test]
    fn print() {
        // 1 创建 Standard Printer
        let out = termcolor::StandardStream::stdout(ColorChoice::Auto);
        let mut builder = StandardBuilder::new();
        builder.max_columns(Option::Some(4096))
            .trim_ascii(true);
        let mut standard = builder.build(out);

        // 2 假设经过Matcher匹配到的行和匹配的字符串
        let bytes = "software to the public domain. We make this dedication for the benefit".as_bytes();
        let binding = vec!(Match::new(25, 29));
        let matches = binding.as_slice();
        // let mut line = Match::new(0, bytes.len());

        // 3 输出
        let sink = standard.sink();
        let standard_impl = StandardImpl::new(&sink);
        standard_impl.write_colored_line(matches, bytes).unwrap();
    }
}