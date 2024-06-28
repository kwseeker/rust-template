use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    iter::Skip,
};
use anyhow::Context;
use lexopt::Arg;
use log::{debug, info};
use crate::options::{
    defs::FLAGS,
    Flag,
    hiargs::HiArgs,
    lowargs::LowArgs,
};
use crate::options::lowargs::{LoggingMode, SpecialMode};

#[derive(Debug)]
struct Parser {
    //命令行选项的索引Map, (ch -> u8) -> index in vec
    map: FlagMap,
    //命令行选项的向量， index -> FlagInfo
    info: Vec<FlagInfo>,
}

impl Parser {
    //创建并初始化命令行参数Parser，主要就是初始化内部的命令行选项Map
    fn new() -> &'static Parser {   //返回一个具有全局生命周期的引用
        use std::sync::OnceLock;
        static P: OnceLock<Parser> = OnceLock::new();   //静态变量只会在程序启动时初始化一次
        P.get_or_init(|| {                              //OnceLock 控制的变量只会写入一次（且是同步写入）
            let mut infos = vec![];
            //命令行选项数据全部存储在FlAGS这个常量中, 遍历将它们加入到 FlagMap 和 Vec<FlagInfo> 这种可检索的数据结构中
            // for &flag in FLAGS.iter() {
            for &flag in FLAGS {
                infos.push(FlagInfo {
                    flag,
                    name: Ok(flag.name_long()),
                    kind: FlagInfoKind::Standard,
                });
                // for alias in flag.aliases() {               //别名选项当作单独的FlagInfo
                //     infos.push(FlagInfo {
                //         flag,
                //         name: Ok(alias),
                //         kind: FlagInfoKind::Alias,
                //     });
                // }
                if let Some(byte) = flag.name_short() {     //短选项当作单独的FlagInfo
                    infos.push(FlagInfo {
                        flag,
                        name: Err(byte),
                        kind: FlagInfoKind::Standard,
                    });
                }
                if let Some(name) = flag.name_negated() {   //否定选项当作单独的FlagInfo
                    infos.push(FlagInfo {
                        flag,
                        name: Ok(name),
                        kind: FlagInfoKind::Negated,
                    });
                }
            }
            let map = FlagMap::new(&infos); //建立索引
            Parser { map, info: infos }
        })
    }

    //将命令行参数解析为 LowArgs
    fn parse<I, O>(&self, raw_args: I, args: &mut LowArgs) -> anyhow::Result<()>
    where
        I: IntoIterator<Item=O>,
        O: Into<OsString>,
    {
        let mut parser = lexopt::Parser::from_args(raw_args);
        //实际的参数解析流程
        while let Some(arg) = parser.next().context("invalid CLI arguments")? {   // next() 获取下一个参数的Result实例、context() 设置错误拓展信息
            let lookup = match arg {
                //匹配 -h / --help
                Arg::Short(ch) if ch == 'h' => {                //Rust 模式匹配规则，参考 tests/grammar/patterns_tests.rs
                    args.special = Some(SpecialMode::HelpShort);
                    continue;
                }
                Arg::Long(name) if name == "help" => {
                    args.special = Some(SpecialMode::HelpLong);
                    continue;
                }
                //匹配 -V / --version
                Arg::Short(ch) if ch == 'V' => {
                    args.special = Some(SpecialMode::VersionShort);
                    continue;
                }
                Arg::Long(name) if name == "version" => {
                    // Special case -V/--version since behavior is different
                    // based on whether short or long flag is given.
                    args.special = Some(SpecialMode::VersionLong);
                    continue;
                }
                //其他参数，其实上面单独匹配的参数可以删除，因为在FLAGS中已经加上了对应选项的Flag实现
                Arg::Short(ch) => self.find_short(ch),
                Arg::Long(name) => self.find_long(name),
                Arg::Value(value) => {
                    args.positional.push(value);
                    continue;
                }
            };
        }
        Ok(())
    }

    /// 从 Parser map info 中检索 Flag 选项实现
    fn find_short(&self, ch: char) -> FlagLookup<'_> {  //TODO '_ 的作用？
        if !ch.is_ascii() { //不支持非ASCII字符
            return FlagLookup::UnrecognizedShort(ch);
        }
        let byte = u8::try_from(ch).unwrap();   //char -> u8
        let Some(index) = self.map.find(&[byte]) else {
            return FlagLookup::UnrecognizedShort(ch);
        };
        FlagLookup::Match(&self.info[index])
    }

    fn find_long(&self, name: &str) -> FlagLookup<'_> {
        FlagLookup::UnrecognizedLong(String::from(name))
    }
}

#[derive(Debug)]
pub(crate) enum ParseResult<T> {
    Special(SpecialMode),
    Ok(T),
    Err(anyhow::Error),
}

impl<T> ParseResult<T> {
    /// 针对非特殊选项、非错误解析结果，传递一个闭包做进一步处理
    /// 对于特殊选项、错误解析结果则原样返回
    fn and_then<U>(self, mut then: impl FnMut(T) -> ParseResult<U>) -> ParseResult<U> { //注意这里第一个参数是self
        match self {
            ParseResult::Special(mode) => ParseResult::Special(mode),
            ParseResult::Ok(t) => then(t),
            ParseResult::Err(err) => ParseResult::Err(err),
        }
    }
}

// 解析命令行参数到 LowArgs 然后转换成 HiArgs 类实例
pub(crate) fn parse() -> ParseResult<HiArgs> {
    //读取命令行参数
    let argv: Vec<OsString> = env::args_os().skip(1).collect();    // collect() 将 Skip<ArgsOs> 转 Vec<OsString>
    print_args(argv.iter().cloned());   //TODO: 这里的原理
    //Parser 解析命令行参数为 LowArgs， 再根据选项类型决定是否进行进步解析
    parse_low(argv.iter().cloned())
        .and_then(|low| match HiArgs::from_low_args(low) {
            Ok(hi) => ParseResult::Ok(hi),
            Err(err) => ParseResult::Err(err),
        })
}

fn print_args<I: IntoIterator<Item=OsString>>(args: I) {
    let mut arguments = String::new();
    for arg in args {
        if !arguments.is_empty() {
            arguments.push_str(" ");
        }
        arguments.push_str(arg.to_string_lossy().into_owned().as_str());    //TODO Cow<>
    }
    info!("arguments: {arguments}");    //TODO Rust 怎么通过修改配置而不是源码的方式修改日志输出级别？
}

fn parse_low<I, O>(raw_args: I) -> ParseResult<LowArgs>
where
    I: IntoIterator<Item=O>,
    O: Into<OsString>,
{
    let parser = Parser::new();
    let mut low = LowArgs::default();
    if let Err(err) = parser.parse(raw_args, &mut low) {
        return ParseResult::Err(err);
    }

    //设置日志输出级别，ripgrep 是将日志级别控制也加入到了命令行选项中
    set_log_levels(&low);

    //特殊选项处理
    if let Some(special) = low.special.take() { //take() 方法会返回Option中的值并替换为None，即取走
        return ParseResult::Special(special);
    }

    return ParseResult::Ok(low);
}

/// FLAGS 有一个命令行参数，用于设置日志输出级别
fn set_log_levels(low: &LowArgs) {
    match low.logging {
        Some(LoggingMode::Trace) => {
            log::set_max_level(log::LevelFilter::Trace)
        }
        Some(LoggingMode::Debug) => {
            log::set_max_level(log::LevelFilter::Debug)
        }
        None => log::set_max_level(log::LevelFilter::Warn)      //默认级别是警告
    }
}

//命令行选项索引的Map
#[derive(Debug)]
struct FlagMap {
    // Vec<u8> -> usize, 即 选项名字 -> 选项在 info 中的索引
    map: HashMap<Vec<u8>, usize>,
}

impl FlagMap {
    fn new(infos: &[FlagInfo]) -> FlagMap { //将向量引用转换为切片引用，因为 Vec 实现了 Deref trait，它可以被解引用为一个切片。 TODO: Deref
        let mut m = HashMap::new();
        let mut idx = 0usize;
        let mut vec;
        for flag_info in infos {
            match flag_info.name {
                Ok(name) => {           //选项完整名、否定名
                    vec = name.as_bytes().to_vec();
                }
                Err(short_name) => {     //短选项名
                    vec = vec![short_name];
                }
            }
            m.insert(vec, idx);
            idx += 1;
        }
        FlagMap { map: m }
    }

    fn find(&self, idx: &[u8]) -> Option<usize> {
        Some(0usize)
    }
}

#[derive(Debug)]
enum FlagLookup<'a> {
    Match(&'a FlagInfo),
    UnrecognizedShort(char),
    UnrecognizedLong(String),
}

/// 命令行选项信息类
#[derive(Debug)]
struct FlagInfo {
    flag: &'static dyn Flag,
    /// 使用Ok存储选项完整名称，使用Err存储选项的单字节短名称
    name: Result<&'static str, u8>,
    /// 选项类型
    kind: FlagInfoKind,
}

#[derive(Debug)]
enum FlagInfoKind {
    /// 标准选项，比如 --multiline
    Standard,
    /// 标准选项的否定选项，比如 --no-multiline
    Negated,
    /// 选项别名
    Alias,
}
