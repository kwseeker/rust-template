// use 其实是创建了成员的快捷方式，用来减少路径的重复，其实可以完全不用 use 但是就会使得引用很长
// 引入标准库中的类，标准库中部分类是预导入的不需要声明，这里显式导入的都是未预导入的
use std::{io::Write, process::ExitCode};
//
use lexopt::{prelude::*};

// 得益于 options/mod.rs 的 "pub(crate) use crate::options::hiargs::HiArgs" 这里才可以写的短一些
use crate::options::{HiArgs, ParseResult};

// mod options 表示从 options.rs 或 options/mod.rs 中查找模块代码
// 这里是通过 options 模块的 mod.rs 以及 mod.rs 中通过 mod 声明的模块，将全部代码联系起来的
mod options;
mod log;

// Rust 函数定义格式 fn function_name(parameters) -> return_type {...}
fn main() -> ExitCode {
    let result: ParseResult<HiArgs> = options::parse();

    return ExitCode::SUCCESS;
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