use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::path::Path;
use grep_matcher::{LineTerminator, Matcher};
use crate::line_buffer::{LineBuffer, LineBufferReader};
use crate::searcher::glue::ReadByLine;
use crate::sink::{Sink, SinkError};

mod glue;

#[derive(Clone, Debug)]
pub struct Config {
    multi_line: bool
}

impl Default for Config {
    fn default() -> Config {
        Config {
            multi_line: false,
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
        //
        let mut decode_builder = DecodeReaderBytesBuilder::new();

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
    // decode_builder: DecodeReaderBytesBuilder,
    /// 用于转码暂存空间的缓冲区
    // decode_buffer: RefCell<Vec<u8>>,
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

    ///
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
        //创建 LineBufferReader 用于将文件内容读取到缓冲, ripgrep 这里还拓展了 read_from 用于支持对多种编码格式文件的读，不过这里暂不需要先忽略
        let mut line_buffer = self.line_buffer.borrow_mut();
        let rdr = LineBufferReader::new(read_from, &mut *line_buffer);

        log::trace!("generic reader: searching via roll buffer strategy");
        ReadByLine::new(self, matcher, rdr, write_to).run()
    }
}