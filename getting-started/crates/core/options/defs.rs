use bstr::ByteVec;
use crate::options::{Category, Flag, FlagValue};
use crate::options::lowargs::{CaseMode, LoggingMode, LowArgs, PatternSource};

/// 这里使用常量存储了所有预设的命令行选项
/// ripgrep 支持很多命令行选项，提供了非常丰富的功能，但是这里只展示几种常用的选项
pub(super) const FLAGS: &[&dyn Flag] = &[   //FLAGS是一个数组引用，数组中存储的是各种命令行选项的引用
    &Help,
    &Version,
    &Debug,
    &IgnoreCase,
    &CaseSensitive,
    &SmartCase,
    // &Color,
    &Column,
    &Heading,
    &LineNumber,
    &LineNumberNo,
    &PathSeparator,
    &Regexp,
    &Threads,
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

    fn update(&self, value: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.logging = Some(LoggingMode::Debug);
        Ok(())
    }
}

/// -i/--ignore-case
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

    fn update(&self, value: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(value.unwrap_switch(), "flag has no negation");
        args.case = CaseMode::Insensitive;
        Ok(())
    }
}

/// -s/--case-sensitive
/// 是否对大小写敏感
#[derive(Debug)]
struct CaseSensitive;

impl Flag for CaseSensitive {
    fn name_long(&self) -> &'static str {
        "case-sensitive"
    }

    fn name_short(&self) -> Option<u8> {
        Some(b's')
    }

    fn doc_category(&self) -> Category {
        Category::Search
    }

    fn doc_short(&self) -> &'static str {
        r"Search case sensitively (default)."
    }

    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "flag has no negation");
        args.case = CaseMode::Sensitive;
        Ok(())
    }
}

/// -S/--smart-case
#[derive(Debug)]
struct SmartCase;

impl Flag for SmartCase {
    fn name_long(&self) -> &'static str {
        "smart-case"
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'S')
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        r"Smart case search."
    }
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--smart-case flag has no negation");
        args.case = CaseMode::Smart;
        Ok(())
    }
}

// --color
// #[derive(Debug)]
// struct Color;
//
// impl Flag for Color {
//     fn name_long(&self) -> &'static str {
//         "color"
//     }
//     fn doc_category(&self) -> Category {
//         Category::Output
//     }
//     fn doc_short(&self) -> &'static str {
//         "When to use color."
//     }
//     fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
//         args.color = match convert::str(&v.unwrap_value())? {
//             "never" => ColorChoice::Never,
//             "auto" => ColorChoice::Auto,
//             "always" => ColorChoice::Always,
//             unk => anyhow::bail!("choice '{unk}' is unrecognized"),
//         };
//         Ok(())
//     }
//     fn doc_variable(&self) -> Option<&'static str> {
//         Some("WHEN")
//     }
//     fn doc_choices(&self) -> &'static [&'static str] {
//         &["never", "auto", "always", "ansi"]
//     }
// }

/// --column
#[derive(Debug)]
struct Column;

impl Flag for Column {
    fn name_long(&self) -> &'static str {
        "column"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-column")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        "Show column numbers."
    }
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.column = Some(v.unwrap_switch());
        Ok(())
    }
}

/// --heading
#[derive(Debug)]
struct Heading;

impl Flag for Heading {
    fn name_long(&self) -> &'static str {
        "heading"
    }
    fn name_negated(&self) -> Option<&'static str> {
        Some("no-heading")
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        r"Print matches grouped by each file."
    }
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        args.heading = Some(v.unwrap_switch());
        Ok(())
    }
}

/// -n/--line-number
#[derive(Debug)]
struct LineNumber;

impl Flag for LineNumber {
    fn name_long(&self) -> &'static str {
        "line-number"
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'n')
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        r"Show line numbers."
    }
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(v.unwrap_switch(), "--line-number has no automatic negation");
        args.line_number = Some(true);
        Ok(())
    }
}

/// -N/--no-line-number
#[derive(Debug)]
struct LineNumberNo;

impl Flag for LineNumberNo {
    fn name_long(&self) -> &'static str {
        "no-line-number"
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'N')
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        r"Suppress line numbers."
    }
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        assert!(
            v.unwrap_switch(),
            "--no-line-number has no automatic negation"
        );
        args.line_number = Some(false);
        Ok(())
    }
}

/// --path-separator
#[derive(Debug)]
struct PathSeparator;

impl Flag for PathSeparator {
    fn name_long(&self) -> &'static str {
        "path-separator"
    }
    fn doc_category(&self) -> Category {
        Category::Output
    }
    fn doc_short(&self) -> &'static str {
        r"Set the path separator for printing paths."
    }
    fn is_switch(&self) -> bool {
        false
    }
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let s = convert::string(v.unwrap_value())?;
        let raw = Vec::unescape_bytes(&s);
        args.path_separator = if raw.is_empty() {
            None
        } else if raw.len() == 1 {
            Some(raw[0])
        } else {
            anyhow::bail!(
                "A path separator must be exactly one byte, but \
                 the given separator is {len} bytes: {sep}\n\
                 In some shells on Windows '/' is automatically \
                 expanded. Use '//' instead.",
                len = raw.len(),
                sep = s,
            )
        };
        Ok(())
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("SEPARATOR")
    }
}

/// -e/--regexp
/// 基于正则表达式进行匹配查找
#[derive(Debug)]
struct Regexp;

impl Flag for Regexp {
    fn name_long(&self) -> &'static str {
        "regexp"
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'e')
    }
    fn doc_category(&self) -> Category {
        Category::Input
    }
    fn doc_short(&self) -> &'static str {
        r"A pattern to search for."
    }
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let regexp = convert::string(v.unwrap_value())?;
        args.patterns.push(PatternSource::Regexp(regexp));
        Ok(())
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("PATTERN")
    }
}

// -r/--replace
// 匹配后替换
// #[derive(Debug)]
// struct Replace;

/// -j/--threads
#[derive(Debug)]
struct Threads;

impl Flag for Threads {
    fn name_long(&self) -> &'static str {
        "threads"
    }
    fn name_short(&self) -> Option<u8> {
        Some(b'j')
    }
    fn doc_category(&self) -> Category {
        Category::Search
    }
    fn doc_short(&self) -> &'static str {
        r"Set the approximate number of threads to use."
    }
    fn is_switch(&self) -> bool {
        false
    }
    fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
        let threads = convert::usize(&v.unwrap_value())?;
        args.threads = if threads == 0 { None } else { Some(threads) };
        Ok(())
    }
    fn doc_variable(&self) -> Option<&'static str> {
        Some("NUM")
    }
}

mod convert {
    use std::ffi::{OsStr, OsString};
    use anyhow::Context;

    /// OsStr 转 &str
    pub(super) fn str(v: &OsStr) -> anyhow::Result<&str> {
        let Some(s) = v.to_str() else {
            anyhow::bail!("value is not valid UTF-8")
        };
        Ok(s)
    }

    /// OsString 转 String
    pub(super) fn string(v: OsString) -> anyhow::Result<String> {
        let Ok(s) = v.into_string() else {
            anyhow::bail!("value is not valid UTF-8")
        };
        Ok(s)
    }

    /// str 转 usize
    pub(super) fn usize(v: &OsStr) -> anyhow::Result<usize> {
        str(v)?.parse().context("value is not a valid number")
    }

    /// str 转 u64
    pub(super) fn u64(v: &OsStr) -> anyhow::Result<u64> {
        str(v)?.parse().context("value is not a valid number")
    }
}
