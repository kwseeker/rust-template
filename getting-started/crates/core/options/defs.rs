use crate::options::{Category, Flag};

/// 这里使用常量存储了所有预设的命令行选项
/// ripgrep 支持很多命令行选项，提供了非常丰富的功能，但是这里只展示几种常用的选项
pub(super) const FLAGS: &[&dyn Flag] = &[   //FLAGS是一个数组引用，数组中存储的是各种命令行选项的引用
    &Help,
    &Version,
    &Debug,
    &IgnoreCase,
];

/// -h/--help
#[derive(Debug)]
struct Help;

impl Flag for Help {
    fn name_long(&self) -> &'static str {
        "help"
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'h')  //b开头的字符称为字节字面量，值为字符对应的ASCII码值
    }

    fn doc_category(&self) -> Category {
        Category::Output
    }

    fn doc_short(&self) -> &'static str {
        r"Show help output."  //这种是原生字符串
    }
}

/// --version
/// 查看当前工具版本
#[derive(Debug)]
struct Version;

impl Flag for Version {
    fn name_long(&self) -> &'static str {
        "version"
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'V')  //b开头的字符称为字节字面量，值为字符对应的ASCII码值
    }

    fn doc_category(&self) -> Category {
        Category::OtherBehaviors
    }

    fn doc_short(&self) -> &'static str {
        r"Print gs's version."  //这种是原生字符串
    }
}

/// --debug
/// 用于设置日志输出级别
#[derive(Debug)]
struct Debug;

impl Flag for Debug {
    fn name_long(&self) -> &'static str {
        "debug"
    }

    fn doc_category(&self) -> Category {
        Category::Logging
    }

    fn doc_short(&self) -> &'static str {
        r"Show debug messages."
    }
}

/// -i/--ignore-case
/// 忽略大小写
#[derive(Debug)]
struct IgnoreCase;

impl Flag for IgnoreCase {
    fn name_long(&self) -> &'static str {
        "ignore-case"
    }

    fn name_short(&self) -> Option<u8> {
        Some(b'i')
    }

    fn doc_category(&self) -> Category {
        Category::Search
    }

    fn doc_short(&self) -> &'static str {
        r"Case insensitive search."
    }
}

/// -s/--case-sensitive
/// 大小写敏感搜索
#[derive(Debug)]
struct CaseSensitive;

/// -n/--line-number
/// 展示匹配行的行号
#[derive(Debug)]
struct LineNumber;

/// -r/--replace
/// 匹配后替换
#[derive(Debug)]
struct Replace;

/// -j/--threads
#[derive(Debug)]
struct Threads;

/// -a/--text
#[derive(Debug)]
struct Text;

/// --sort
#[derive(Debug)]
struct Sort;

/// -m/--max-count
#[derive(Debug)]
struct MaxCount;

/// -e/--regexp
/// 基于正则表达式进行匹配查找
#[derive(Debug)]
struct Regexp;

/// --binary
#[derive(Debug)]
struct Binary;