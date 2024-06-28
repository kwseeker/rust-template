use std::ffi::OsString;

// 低级参数，可以理解为是原生态的参数
#[derive(Debug, Default)]   //Default为结构体自动派生构造函数
pub(crate) struct LowArgs {
    //特殊选项（查看帮助、查看版本号）的模式
    pub(crate) special: Option<SpecialMode>,
    //
    pub(crate) positional: Vec<OsString>,
    //控制日志输出级别的选项
    pub(crate) logging: Option<LoggingMode>,
}

impl LowArgs {
}

//处理特殊命令行参数（查看帮助和查看版本号）
#[derive(Clone, Copy, Debug, Eq, PartialEq)]    //TODO Clone Copy Eq PartialEq 特征测试
pub(crate) enum SpecialMode {
    HelpShort,
    HelpLong,
    VersionShort,
    VersionLong,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum LoggingMode {
    Debug,
    Trace,
}