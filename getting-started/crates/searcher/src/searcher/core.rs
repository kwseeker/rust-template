use grep_matcher::{LineMatchKind, Matcher};
use crate::searcher::{Config, Range};
use crate::{lines, Searcher, Sink};
use crate::lines::LineStep;
use crate::sink::{SinkError, SinkMatch};

enum FastMatchResult {
    Continue,
    Stop,
}

#[derive(Debug)]
pub(crate) struct Core<'s, M: 's, S> {
    config: &'s Config,
    matcher: M,
    searcher: &'s Searcher,
    sink: S,
    /// 上次匹配到的行的行号，每次成功匹配到行后后累加
    line_number: Option<u64>,
    /// 对单个文件总处理字节数的计数
    absolute_byte_offset: u64,
    /// 上次匹配到的行的开头偏移位置（start）
    last_line_counted: usize,
    /// 上次匹配到的行的结尾偏移位置（end）
    last_line_visited: usize,
    /// 和 LineBufferReader 中的 pos 类似，但是是记录的可读位置，LineBufferReader中pos记录可写位置
    /// 因为从缓冲中匹配数据是一个个匹配的，所以需要记录每次匹配读取到了哪个位置
    pos: usize,
    /// 是否有匹配的行
    has_matched: bool,
    ///
    after_context_left: usize,
}

impl<'s, M: Matcher, S: Sink> Core<'s, M, S> {
    ///
    pub(crate) fn new(
        searcher: &'s Searcher,
        matcher: M,
        sink: S,
        binary: bool,
    ) -> Core<'s, M, S> {
        let line_number = if searcher.config.line_number { Some(1) } else { None };
        let core = Core {
            config: &searcher.config,
            matcher,
            searcher,
            sink,
            pos: 0,
            absolute_byte_offset: 0,
            line_number,
            last_line_counted: 0,
            last_line_visited: 0,
            has_matched: false,
            after_context_left: 0,
        };
        core
    }

    pub(crate) fn begin(&self) -> Result<bool, S::Error>  {
        //TODO
        Ok(true)
    }

    /// roll滚动的意思，这里是指指针的移动
    pub(crate) fn roll(&mut self, buf: &[u8]) -> usize {
        //上次消费的数据字节数
        let consumed = if self.config.max_context() == 0 {  //TODO ???
            buf.len()
        } else {
            let context_start = lines::preceding(   // 返回 buf 中从右往左数第 count+1 行的起始偏移
                buf,
                self.config.line_terminator.as_byte(),
                self.config.max_context(),  //count
            );
            let consumed = std::cmp::max(context_start, self.last_line_visited);
            consumed
        };
        // 更新 line_number last_line_counted
        self.count_lines(buf, consumed);
        self.absolute_byte_offset += consumed as u64;
        self.last_line_counted = 0;
        self.last_line_visited = 0;
        self.set_pos(buf.len() - consumed);
        consumed
    }

    fn count_lines(&mut self, buf: &[u8], upto: usize) {
        if let Some(ref mut line_number) = self.line_number {
            if self.last_line_counted >= upto {
                return;
            }
            let slice = &buf[self.last_line_counted..upto];
            // 计算上次处理的行数，即统计读取的数据中有多少个行终止符
            let count = lines::count(slice, self.config.line_terminator.as_byte());
            *line_number += count;
            self.last_line_counted = upto;
        }
    }

    fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    ///
    pub(crate) fn match_by_line(
        &mut self,
        buf: &[u8],
    ) -> Result<bool, S::Error> {
        match self.match_by_line_fast(buf)? {
            //TODO
            FastMatchResult::Continue => {Ok(true)}
            FastMatchResult::Stop => {Ok(false)}
        }
    }

    /// 查找匹配的行，并输出到 Sink
    fn match_by_line_fast(
        &mut self,
        buf: &[u8],
    ) -> Result<FastMatchResult, S::Error> {
        while !buf[self.pos..].is_empty() { //缓冲中仍然有数据可读
            // 1 查找 buf[self.pos..] 中匹配的行（返回行在buf中范围Range）
            if let Some(line) = self.find_by_line_fast(buf)? {
                self.has_matched = true;
                if self.config.max_context() > 0 {  //TODO ???
                    if !self.after_context_by_line(buf, line.start())? ||
                        !self.before_context_by_line(buf, line.start())? {
                        return Ok(FastMatchResult::Stop);
                    }
                }
                self.set_pos(line.end());

                // 2 将匹配的行输出到 Sink
                if !self.sink_matched(buf, &line)? {
                    return Ok(FastMatchResult::Stop);
                }
            } else {
                break;
            }
        }
        //TODO
        Ok(FastMatchResult::Continue)
    }

    /// 从 LineBufferReader 缓冲 [pos..] 中查找匹配的行
    #[inline(always)]
    fn find_by_line_fast(
        &mut self,
        buf: &[u8],
    ) -> Result<Option<Range>, S::Error> {  //这里 Range 即 Match 的别名类型
        while !buf[self.pos..].is_empty() {
            //每次调用如果成功查找到匹配行，会返回匹配字符串的结尾在缓冲中的位置
            return match self.matcher.find_candidate_line(&buf[self.pos..]) {
                Err(err) => Err(S::Error::error_message(err)),
                Ok(None) => Ok(None),
                Ok(Some(LineMatchKind::Confirmed(i))) => {
                    // Confirmed 中的值是找到的第一个匹配项的结尾在缓冲中的位置
                    // 然后需要根据这个位置，查找到完整行在buf中的范围（范围使用Match对象表示）
                    let line = lines::locate(buf, self.config.line_terminator.as_byte(), Range::zero(i).offset(self.pos));
                    if line.start() == buf.len() {  //不太可能吧
                        self.pos = buf.len();
                        continue;
                    }
                    Ok(Some(line))
                }
                Ok(Some(LineMatchKind::Candidate(i))) => {
                    //TODO 配置了 fast_line_regex 才可能返回这种结果
                    Ok(None)
                }
            }
        }
        Ok(None)
    }

    ///
    pub(crate) fn before_context_by_line(
        &mut self,
        buf: &[u8],
        upto: usize,
    ) -> Result<bool, S::Error> {
        // if self.config.before_context == 0 {
        //     return Ok(true);
        // }
        // let range = Range::new(self.last_line_visited, upto);
        // if range.is_empty() {
        //     return Ok(true);
        // }
        // let before_context_start = range.start()
        //     + lines::preceding(
        //     &buf[range],
        //     self.config.line_terminator.as_byte(),
        //     self.config.before_context - 1,
        // );
        //
        // let range = Range::new(before_context_start, range.end());
        // let mut stepper = LineStep::new(
        //     self.config.line_terminator.as_byte(),
        //     range.start(),
        //     range.end(),
        // );
        // while let Some(line) = stepper.next_match(buf) {
        //     if !self.sink_break_context(line.start())? {
        //         return Ok(false);
        //     }
        //     if !self.sink_before_context(buf, &line)? {
        //         return Ok(false);
        //     }
        // }
        Ok(true)
    }

    /// TODO ?
    pub(crate) fn after_context_by_line(
        &mut self,
        buf: &[u8],
        upto: usize,
    ) -> Result<bool, S::Error> {
        // if self.after_context_left == 0 {
        //     return Ok(true);
        // }
        // let range = Range::new(self.last_line_visited, upto);
        // let mut stepper = LineStep::new(
        //     self.config.line_terminator.as_byte(),
        //     range.start(),
        //     range.end(),
        // );
        // while let Some(line) = stepper.next_match(buf) {
        //     if !self.sink_after_context(buf, &line)? {
        //         return Ok(false);
        //     }
        //     if self.after_context_left == 0 {
        //         break;
        //     }
        // }
        Ok(true)
    }

    /// 将匹配的行通过 Sink 输出
    #[inline(always)]
    fn sink_matched(
        &mut self,
        buf: &[u8],
        range: &Range,
    ) -> Result<bool, S::Error> {
        // 更新 line_counter last_line_counted absolute_byte_offset
        self.count_lines(buf, range.start());
        let offset = self.absolute_byte_offset + range.start() as u64;

        let line_buf = &buf[*range];
        // 输出匹配的行
        let keep_going = self.sink.matched(
            &self.searcher,
            &SinkMatch {
                line_term: self.config.line_terminator,
                bytes: line_buf,
                absolute_byte_offset: offset,
                line_number: self.line_number,
                buffer: buf,
                bytes_range_in_buffer: range.start()..range.end(),
            })?;

        Ok(true)
    }
}