use std::io;
use crate::searcher::{ConfigError};

/// Sink 代表输出的意思
pub trait Sink {
    /// 将 SinkError 重命名为 Error, 然后此 Sink 内部使用 Error 实际就是使用的 SinkError
    type Error: SinkError;


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
}
