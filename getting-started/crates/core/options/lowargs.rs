use std::ffi::OsString;

// 低级参数，可以理解为是原生态的参数
#[derive(Debug, Default)]   //Default为结构体自动派生构造函数
pub(crate) struct LowArgs {
    //特殊选项（查看帮助、查看版本号）的模式
    pub(crate) special: Option<SpecialMode>,
    //
    pub(crate) positional: Vec<OsString>
}

impl LowArgs {
    // pub(crate) fn new(val: &str) -> LowArgs {
    //     LowArgs {
    //     }
    // }
}

//处理特殊命令行参数（查看帮助和查看版本号）
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpecialMode {
    HelpShort,
    HelpLong,
    VersionShort,
    VersionLong,
}