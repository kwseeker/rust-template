use std::ffi::OsString;

/// LowArgs 低级参数，里面大部分成员字段对应FLAGS中的选项，LowArgs对象是命令行参数经过和Parser中map info匹配后
/// 实际需要使用的参数中间对象（只有一些选项配置的基础信息）
#[derive(Debug, Default)]   //Default为结构体自动派生构造函数
pub(crate) struct LowArgs {
    //特殊选项（查看帮助、查看版本号）的模式
    pub(crate) special: Option<SpecialMode>,
    //命名行参数（比如 gs [OPTIONS] PATTERN [PATH ...] 中的 PATTERN、PATH）
    pub(crate) positional: Vec<OsString>,
    //控制日志输出级别的选项
    pub(crate) logging: Option<LoggingMode>,
    pub(crate) mode: Mode,
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

/// 工作模式，ripgrep 默认支持四种模式，这里只展示 Search 和 Files
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Mode {
    /// 执行搜索，ripgrep 搜索模式有多种 Standard | FilesWithMatches | FilesWithoutMatch | Count | CountMatches | JSON,
    /// 这里先不管，只看标准模式
    Search(SearchMode),
    /// 展示会搜索的文件列表，但是并不会执行搜索
    Files,
    /// 列出配置的所有文件类型定义，包括默认文件类型和添加到命令行的任何其他文件类型
    // Types,
    /// 用于生成帮助等文件（man page and completion files）
    // Generate(GenerateMode),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Search(SearchMode::Standard)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SearchMode {
    Standard,
}