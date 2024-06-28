use crate::options::Flag;

/// 这里使用常量存储了所有预设的命令行选项
/// ripgrep 支持很多命令行选项，提供了非常丰富的功能，但是这里只展示几种常用的选项
pub(super) const FLAGS: &[&dyn Flag] = &[   //FLAGS是一个数组引用，数组中存储的是各种命令行选项的引用
    &Help,
    &Version,
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

    fn doc_short(&self) -> &'static str {
        r"Print gs's version."  //这种是原生字符串
    }
}

/// -i/--ignore-case
#[derive(Debug)]
struct IgnoreCase;

/// -s/--case-sensitive
/// 是否对大小写敏感
#[derive(Debug)]
struct CaseSensitive;

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

/// -n/--line-number
#[derive(Debug)]
struct LineNumber;

/// -e/--regexp
/// 基于正则表达式进行匹配查找
#[derive(Debug)]
struct Regexp;

/// --binary
#[derive(Debug)]
struct Binary;