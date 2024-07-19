// use 其实是创建了成员的快捷方式，用来减少路径的重复，其实可以完全不用 use 但是就会使得引用很长
// 引入标准库中的类，标准库中部分类是预导入的不需要声明，这里显式导入的都是未预导入的
use std::{io::Write, process::ExitCode};
use std::path::Path;
use anyhow::anyhow;
use log::info;
use lexopt::{prelude::*};
use termcolor::ColorChoice;
use grep::printer::StandardBuilder;
use grep::regex::RegexMatcherBuilder;
use grep::searcher::SearcherBuilder;

use crate::logger::logger::Logger;
// 得益于 options/mod.rs 的 "pub(crate) use crate::options::hiargs::HiArgs" 这里才可以写的短一些
use crate::options::{HiArgs, ParseResult};
use crate::options::lowargs::{Mode, SearchMode, SpecialMode};
use crate::search::{PatternMatcher, Printer};

// mod options 表示从 options.rs 或 options/mod.rs 中查找模块代码
// 这里是通过 options 模块的 mod.rs 以及 mod.rs 中通过 mod 声明的模块，将全部代码联系起来的
mod options;
mod logger;
mod search;

/// 二进制执行，比如: gs -i --debug grep ./crates/grep
/// RustRover执行，配置 Command: run --package getting-started --bin gs -- -i --debug grep ./crates/grep
// Rust 函数定义格式 fn function_name(parameters) -> return_type {...}
fn main() -> ExitCode {
    //1 初始化日志
    if let Err(err) = logger::logger::init() {
        return ExitCode::FAILURE
    }
    info!("exec init done!");

    //2 命令行解析
    // 首先 ripgrep 中为何支持那么多选项，其实里面很多选项是 ripgrep 中一些依赖库的配置
    // Parser 初始化的时候将 defs.rs 中定义的参数及其详细信息加载到 Parser 内部的 map、info 这种可以通过名称或简写名称索引的数据结构
    // 然后从命令行参数提取命令行选项，通过命令行选项名称检索获取选项详细信息，并替换 LowArgs 对象（里面对应defs.rs中的选项，只不过都是默认配置）中对应选项的默认配置
    // 对于非特殊选项（查看帮助、版本号等）还会转成 HiArgs
    let result: ParseResult<HiArgs> = options::parse();

    //3 选项参数处理，比如执行搜索
    run(result).unwrap_or_else(|err| {
        eprintln_locked!("{:#}", err);
        ExitCode::from(1)
    })
}

/// 走到这里有3种可能结果，对应处理即可
/// Special(SpecialMode),    特殊选项
/// Ok(T),                   非特殊选项
/// Err(anyhow::Error),      解析异常
fn run(result: ParseResult<HiArgs>) -> anyhow::Result<ExitCode> {
    let mut args = match result {
        ParseResult::Err(err) => return Err(err),
        ParseResult::Special(mode) => return special(mode), //特殊选项的处理, 比如查看帮助、查看版本号
        ParseResult::Ok(args) => args,                          //非特殊选项解构
    };

    // 非特殊选项处理，ripgrep 提供了四种工作模式：
    // Search：搜索匹配项
    // Files：列举搜索的目标文件列表但并不执行真正的搜索
    // Types：列举配置的所有文件类型
    // Generate：生成帮助文档等
    // 这里只展示 Search
    let matched = match args.mode() {
        Mode::Search(_) if !args.matches_possible() => false,
        Mode::Search(mode) if args.threads() == 1 => search(&mut args, mode)?,
        Mode::Search(mode) => search_parallel(&args, mode)?,
        Mode::Files => false   //先忽略
    };
    let exit_code = if matched {
        ExitCode::from(0)
    } else {
        ExitCode::from(1)
    };
    Ok(exit_code)
}

fn special(mode: SpecialMode) -> anyhow::Result<ExitCode> {
    let output = match mode {
        SpecialMode::HelpShort => options::generate_help_short(),
        SpecialMode::HelpLong => options::generate_help_short(),
        SpecialMode::VersionShort => options::generate_version_short(),
        SpecialMode::VersionLong => options::generate_version_long(),
    };
    // 打印到标准输出
    writeln!(std::io::stdout(), "{}", output.trim_end())?;
    Ok(ExitCode::SUCCESS)
}

fn search(args: &mut HiArgs, mode: SearchMode) -> anyhow::Result<bool> {
    let mut matched = false;
    //1 创建 SearchWorker
    let mut search_worker = args.search_worker(
        args.matcher()?,
        args.searcher()?,
        args.printer(mode, args.stdout()),
    )?;
    // 2 递归查找
    let paths = args.paths();
    for path_buf in paths {
        // 执行搜索、输出等流程
        let search_result = search_worker.search(path_buf.as_path())?;
        matched = matched || search_result.has_match();
        // ripgrep 还支持统计功能，但是这里不展示了
    }
    Ok(matched)
}

fn search_parallel(args: &HiArgs, mode: SearchMode) -> anyhow::Result<bool> {
    //TODO
    Ok(false)
}

// 条件编译宏，这里表示只有在执行cargo test才会编译和运行tests模块
// Rust单元测试习惯和业务代码放在一起，集成测试则放到tests文件夹
#[cfg(test)]
#[allow(unused_variables, dead_code)]   //允许测试中未使用的变量和未使用的代码
mod rb_tests {  //rust语法基础测试

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    #[test]
    fn match_usage() {
        let coin = Coin::Nickel;
        println!("cents: {}", value_in_cents(coin))
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}