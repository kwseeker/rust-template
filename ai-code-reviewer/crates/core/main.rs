use log::{debug, error, info};
use crate::reviewer::Reviewer;

mod reviewer;
mod openai;
mod github;
mod options;
mod common;
mod init;

/// ai-code-reviewer [OPTION]
/// OPTIONS:
///     --pr-number=[PR_NUMBER]     用于 pull_request 事件
///     --ref=[REF]                 用于 push 事件
fn main() {
    debug!("Execute AI code review ...");
    // 使用 lexopt 解析命令行参数
    let args = options::parse_env_args();
    if args.is_err() {
        panic!("Error: {}", args.unwrap_err());
    }
    let args = args.unwrap();
    // 一些初始化
    init::init(&args);

    let result = Reviewer::new().review(args);
    match result {
        Ok(_) => info!("AI code review done"),
        Err(err) => {
            // eprintln!("Error: {}", err); //这种方式无法打印出错误详细信息， panic!() 可以打印出详细错误信息, 不过是编译器实现的
            // 另外也可以通过 error::Error 的特征方法挖掘错误详细信息
            error!("Error: {err:?}, Cause by {:?}", err.source());
            std::process::exit(1);
        }
    }
}
