use std::io;
use grep_matcher::LineTerminator;
use crate::Searcher;
use crate::searcher::{ConfigError};

/// Sink 代表输出的意思
pub trait Sink {
    /// 将 SinkError 重命名为 Error, 然后此 Sink 内部使用 Error 实际就是使用的 SinkError
    type Error: SinkError;

    /// 找到匹配项后调用
    fn matched(
        &mut self,
        _searcher: &Searcher,
        _mat: &SinkMatch<'_>,
    ) -> Result<bool, Self::Error>;
}

pub trait SinkError: Sized {
    /// 一个构造器用于将任意实现了 `std::fmt::Display` 的 trait 转换为 SinkError
    fn error_message<T: std::fmt::Display>(message: T) -> Self;

    /// 一个构造器用于将搜索过程中发生的IO错误转换为 SinkError
    fn error_io(err: io::Error) -> Self {
        Self::error_message(err)
    }

    /// 一个构造器用于将Searcher构建过程中发生的 ConfigError 转换为 SinkError
    fn error_config(err: ConfigError) -> Self {
        Self::error_message(err)
    }
}

/// 这样 io::Error 对象可以直接赋值给 SinkError 对象
impl SinkError for io::Error {
    fn error_message<T: std::fmt::Display>(message: T) -> io::Error {
        io::Error::new(io::ErrorKind::Other, message.to_string())
    }

    fn error_io(err: io::Error) -> io::Error {
        err
    }
}

impl<'a, S: Sink> Sink for &'a mut S {
    type Error = S::Error;

    fn matched(&mut self, searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, Self::Error> {
        (**self).matched(searcher, mat)
    }
}

/// 用于描述匹配行信息的类型
#[derive(Clone, Debug)]
pub struct SinkMatch<'b> {
    pub(crate) line_term: LineTerminator,
    /// 匹配行的字节数组
    pub(crate) bytes: &'b [u8],
    pub(crate) absolute_byte_offset: u64,
    pub(crate) line_number: Option<u64>,
    /// 读缓冲的字节数组
    pub(crate) buffer: &'b [u8],
    /// 匹配的行在缓冲中的范围
    pub(crate) bytes_range_in_buffer: std::ops::Range<usize>,
}

impl<'b> SinkMatch<'b> {
    #[inline]
    pub fn buffer(&self) -> &'b [u8] {
        self.buffer
    }

    #[inline]
    pub fn bytes_range_in_buffer(&self) -> std::ops::Range<usize> {
        self.bytes_range_in_buffer.clone()
    }

    #[inline]
    pub fn bytes(&self) -> &'b [u8] {
        self.bytes
    }

    #[inline]
    pub fn absolute_byte_offset(&self) -> u64 {
        self.absolute_byte_offset
    }

    #[inline]
    pub fn line_number(&self) -> Option<u64> {
        self.line_number
    }
}