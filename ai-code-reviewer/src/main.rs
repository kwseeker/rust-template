use crate::reviewer::Reviewer;

mod reviewer;
mod openai;
mod github;
mod options;
mod errors;

/// ai-code-reviewer [OPTION]
/// OPTIONS:
///     --pr-number=[PR_NUMBER]     用于 pull_request 事件
///     --ref=[REF]                 用于 push 事件
fn main() {
    println!("Execute AI code review ...");
    // 使用 lexopt 解析命令行参数
    let args = options::parse_env_args();
    if args.is_err() {
        eprintln!("Error: {}", args.unwrap_err());
        std::process::exit(1);
    }

    let result = Reviewer::new().review(args.unwrap());
    match result {
        Ok(_) => println!("AI code review done"),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
