// use 其实是创建了成员的快捷方式，用来减少路径的重复，其实可以完全不用 use 但是就会使得引用很长
// 引入标准库中的类，标准库中部分类是预导入的不需要声明，这里显式导入的都是未预导入的
use std::{io::Write, process::ExitCode};
use anyhow::anyhow;
use log::info;
use lexopt::{prelude::*};

use crate::logger::logger::Logger;
// 得益于 options/mod.rs 的 "pub(crate) use crate::options::hiargs::HiArgs" 这里才可以写的短一些
use crate::options::{HiArgs, ParseResult};
use crate::options::lowargs::SpecialMode;

// mod options 表示从 options.rs 或 options/mod.rs 中查找模块代码
// 这里是通过 options 模块的 mod.rs 以及 mod.rs 中通过 mod 声明的模块，将全部代码联系起来的
mod options;
mod logger;
mod search;

// Rust 函数定义格式 fn function_name(parameters) -> return_type {...}
fn main() -> ExitCode {
    //1 初始化日志
    if let Err(err) = logger::logger::init() {
        return ExitCode::FAILURE
    }
    info!("exec init done!");

    //2 命令行解析
    let result: ParseResult<HiArgs> = options::parse();

    //3 选项参数处理，比如执行搜索
    run(result).unwrap_or_else(|err| {
        eprintln_locked!("{:#}", err);
        ExitCode::from(1)       //TODO
    })
}

/// 走到这里有3种可能结果，对应处理即可
/// Special(SpecialMode),    特殊选项
/// Ok(T),                   非特殊选项
/// Err(anyhow::Error),      解析异常
fn run(result: ParseResult<HiArgs>) -> anyhow::Result<ExitCode> {
    let args = match result {
        ParseResult::Err(err) => return Err(err),
        ParseResult::Special(mode) => return special(mode), //特殊选项的处理
        ParseResult::Ok(args) => args,                          //非特殊选项解构
    };

    //非特殊选项处理

    Ok(ExitCode::SUCCESS)
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