use std::ffi::OsString;

/// 低级参数，可以理解为是原生态的参数
#[derive(Debug, Default)]   //Default为结构体自动派生构造函数
pub(crate) struct LowArgs {
    /// 特殊选项（查看帮助、查看版本号）的模式
    pub(crate) special: Option<SpecialMode>,
    /// positional args 的容器，解析时会先将这类参数通通放到这里, 暂时没仔细研究这部分参数到底什么用
    pub(crate) positional: Vec<OsString>,
    /// 控制日志输出级别的选项
    pub(crate) logging: Option<LoggingMode>,

    /// 工作模式(ripgrep支持四种)，默认工作模式是搜索，默认搜索模式是标准搜素
    pub(crate) mode: Mode,
    /// 匹配使用的 Pattern
    pub(crate) patterns: Vec<PatternSource>,
    /// 大小写是否敏感
    pub(crate) case: CaseMode,
    // 颜色高亮输出颜色选择
    // pub(crate) color: ColorChoice,
    /// 是否打印匹配项在匹配行中的列数
    pub(crate) column: Option<bool>,
    /// 是否以标题的方式打印匹配文件路径
    pub(crate) heading: Option<bool>,
    /// 是否打印匹配行在文件中的行号
    pub(crate) line_number: Option<bool>,
    /// 自定义的路径分隔符
    pub(crate) path_separator: Option<u8>,
    /// 搜索使用线程数量
    pub(crate) threads: Option<usize>,
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

#[derive(Debug)]
pub(crate) enum Mode {
    /// 执行搜索
    Search(SearchMode),
    /// 列举会搜索到的文件列表，但并不真正执行搜索
    Files,
    // Types,
    // Generate(GenerateMode),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Search(SearchMode::Standard)
    }
}

#[derive(Debug)]
pub(crate) enum SearchMode {
    // 标准搜索模式，即搜索路径、文件中匹配行及匹配字段并打印
    Standard,
    // 只展示包含匹配字段的文件
    // FilesWithMatches,
    // 只展示不包含任何匹配字段的文件
    // FilesWithoutMatch,
    // 只展示包含匹配字段的文件的匹配行数量
    // Count,
    // 只展示包含匹配字段的文件的匹配字段的数量
    // CountMatches,
    // 以JSON格式打印匹配项信息
    // JSON,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) enum CaseMode {
    /// 大小写敏感
    #[default]
    Sensitive,
    /// 大小写不敏感
    Insensitive,
    /// 智能模式，只有 pattern 中全是小写字符时才不区分大小写，只要包含大写字符就区分大小写
    Smart
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum PatternSource {
    /// 正则表达式 Pattern
    Regexp(String),
    // File(PathBuf),
}
