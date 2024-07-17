use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::{cmp, io};
use std::path::Path;
use encoding_rs_io::DecodeReaderBytesBuilder;
use grep_matcher::{LineTerminator, Match, Matcher};
use crate::line_buffer::{LineBuffer, LineBufferBuilder, LineBufferReader};
use crate::searcher::glue::ReadByLine;
use crate::sink::{Sink, SinkError};

mod glue;
mod core;

type Range = Match;

#[derive(Clone, Debug)]
pub struct Config {
    /// 使用使用多行模式
    multi_line: bool,
    /// 为 DecodeReaderBytes 显示设置的编码模式，即默认源数据编码模式为 encoding 指定的编码模式
    encoding: Option<Encoding>,
    /// BOM 是字节序标记，可以用于标记字节序，也可以表示编码模式
    bom_sniffing: bool,
    line_terminator: LineTerminator,
    ///
    after_context: usize,
    ///
    before_context: usize,
    /// 是否打印匹配行的行号
    line_number: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            multi_line: false,
            encoding: None,
            bom_sniffing: true,
            line_terminator: LineTerminator::default(),
            after_context: 0,
            before_context: 0,
            line_number: true,
        }
    }
}

impl Config {
    fn line_buffer(&self) -> LineBuffer {
        let mut builder = LineBufferBuilder::new();
        builder
            .line_terminator(self.line_terminator.as_byte()); // ripgrep 还可以设置缓冲容量，先忽略
        builder.build()
    }

    fn max_context(&self) -> usize {
        cmp::max(self.before_context, self.after_context)
    }
}

/// 相当于对 encoding_rs 中的 Encoding 进行重命名
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoding(&'static encoding_rs::Encoding);

impl Encoding {
    /// 根据 label 查找对应的 Encoding 实现
    pub fn new(label: &str) -> Result<Encoding, ConfigError> {
        let label = label.as_bytes();
        match encoding_rs::Encoding::for_label_no_replacement(label) {
            Some(encoding) => Ok(Encoding(encoding)),
            None => {
                Err(ConfigError::UnknownEncoding { label: label.to_vec() })
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SearcherBuilder {
    config: Config,
}

impl Default for SearcherBuilder {
    fn default() -> SearcherBuilder {
        SearcherBuilder::new()
    }
}

impl SearcherBuilder {
    pub fn new() -> SearcherBuilder {
        SearcherBuilder { config: Config::default() }
    }

    pub fn build(&self) -> Searcher {
        let mut config = self.config.clone();

        let mut decode_builder = DecodeReaderBytesBuilder::new();
        decode_builder
            // 当已知所有源数据都是某个编码格式时设置，当为了支持多种编码格式到UTF-8的转换时这个配置就不需要了
            .encoding(self.config.encoding.as_ref().map(|e| e.0))
            // 源数据编码格式是 UTF-8 时直传
            .utf8_passthru(true)
            // 是否清除 BOM 标记位，只有 utf8_passthru(true) 有效，其他情况都会清除 BOM 标记位
            .strip_bom(self.config.bom_sniffing)
            // 修改 BOM 嗅探优先级最高，比如 默认情况 encoding() 显示设置的编码格式优先级高于 BOM 嗅探
            .bom_override(true)
            // 是否开启 BOM 嗅探
            .bom_sniffing(self.config.bom_sniffing);

        Searcher {
            config,
            decode_builder,
            decode_buffer: RefCell::new(vec![0; 8 * (1 << 10)]),    //TODO
            line_buffer: RefCell::new(self.config.line_buffer()),         //TODO
            multi_line_buffer: RefCell::new(vec![]),
        }
    }

    pub fn line_number(&mut self, yes: bool) -> &mut SearcherBuilder {
        self.config.line_number = yes;
        self
    }
}

/// Searcher 构建过程中的错误类型枚举
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfigError {
    /// Indicates that the heap limit configuration prevents all possible
    /// search strategies from being used. For example, if the heap limit is
    /// set to 0 and memory map searching is disabled or unavailable.
    SearchUnavailable,
    /// Occurs when a matcher reports a line terminator that is different than
    /// the one configured in the searcher.
    MismatchedLineTerminators {
        /// The matcher's line terminator.
        matcher: LineTerminator,
        /// The searcher's line terminator.
        searcher: LineTerminator,
    },
    /// Occurs when no encoding could be found for a particular label.
    UnknownEncoding {
        /// The provided encoding label that could not be found.
        label: Vec<u8>,
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Searcher {
    config: Config,
    /// 用于构建流式读取器的 Builder，可以支持不同编码方式的文件的读取和数据编码转换，比如 UTF-16 文件转 UTF-8
    /// 该读取器根据显式指定的编码或通过 BOM 嗅探自动检测到的编码对源数据进行转码。
    /// 当不需要转码时，构建的转码器将传递底层字节，而不会产生额外的开销。
    decode_builder: DecodeReaderBytesBuilder,
    /// 用于转码暂存空间的缓冲区
    decode_buffer: RefCell<Vec<u8>>,
    /// 用于面向行搜索的行缓冲区
    /// 我们将其包装在 RefCell 中，以允许将“Searcher”借用到接收器。
    /// 我们仍然需要可变借用来执行搜索，因此我们静态地防止调用者由于借用违规而导致 RefCell 在运行时出现panic。
    line_buffer: RefCell<LineBuffer>,
    /// 执行多行搜索时用于存储读取器内容的缓冲区。特别是，多行搜索无法增量执行，并且需要一次性将整个干草堆存储在内存中
    multi_line_buffer: RefCell<Vec<u8>>,
}

impl Searcher {
    /// 执行基于文件路径的搜索，并将匹配结果写到给定的sink
    // pub fn search_path<P, M, S>(
    pub fn search_path<M, S>(
        &mut self,
        matcher: M,         // Matcher 实现，比如 RegexMatcher
        path: &Path,        // 文件路径
        write_to: S,        // Sink 实现，比如 StandardSink
    ) -> Result<(), S::Error>
    where
    // P: AsRef<Path>,
        M: Matcher,
        S: Sink,
    {
        let file = File::open(path).map_err(S::Error::error_io)?;
        self.search_file_maybe_path(matcher, Some(path), &file, write_to)
    }

    ///
    fn search_file_maybe_path<M, S>(
        &mut self,
        matcher: M,
        path: Option<&Path>,
        file: &File,
        write_to: S,
    ) -> Result<(), S::Error>
    where
        M: Matcher,
        S: Sink,
    {
        // ripgrep 还支持通过 mmap 读取文件内容 TODO 这里暂时忽略，后面再研究
        // 还支持多行匹配模式 TODO 后面研究

        // 这里展示传统的文件读取方式
        log::trace!("{:?}: searching using generic reader", path);
        self.search_reader(matcher, file, write_to)
    }

    /// 判断是否使用多行匹配模式
    pub fn multi_line_with_matcher<M: Matcher>(&self, matcher: M) -> bool {
        if !self.multi_line() {
            return false;
        }
        //TODO
        true
    }

    #[inline]
    pub fn multi_line(&self) -> bool {
        self.config.multi_line
    }

    /// 即从 io::Read 实现类读取数据并匹配输出
    /// 前两步为 read_from 分别拓展了编码转换、缓冲读的功能
    pub fn search_reader<M, R, S>(
        &mut self,
        matcher: M,
        read_from: R,
        write_to: S,
    ) -> Result<(), S::Error>
    where
        M: Matcher,
        R: io::Read,
        S: Sink,
    {
        // 1 创建编码转换器
        let mut decode_buffer = self.decode_buffer.borrow_mut();
        let decoder = self.decode_builder
            .build_with_buffer(read_from, &mut *decode_buffer)
            .map_err(S::Error::error_io)?;

        // 2 创建 LineBufferReader 用于将文件内容读取到缓冲
        let mut line_buffer = self.line_buffer.borrow_mut();
        // let rdr = LineBufferReader::new(read_from, &mut *line_buffer);
        let rdr = LineBufferReader::new(decoder, &mut *line_buffer);

        // 3
        log::trace!("generic reader: searching via roll buffer strategy");
        ReadByLine::new(self, matcher, rdr, write_to).run()
    }

    pub fn line_terminator(&self) -> LineTerminator {
        self.config.line_terminator
    }
}