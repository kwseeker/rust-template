/// Module std::env https://doc.rust-lang.org/std/env/index.html
/// 该模块包含检查各个方面的函数，例如环境变量、进程参数、当前目录和各种其他重要目录。

///args() 、args_os() 函数返回程序启动时使用的参数(通常通过命令行传递)。
#[test]
fn test_args() {
    println!("version: {}", generate_digits())
}

pub(crate) fn generate_digits() -> String {
    //读取 cargo.toml package.version，没有设置就默认返回 “N/A”
    let semver = option_env!("CARGO_PKG_VERSION").unwrap_or("N/A");
    semver.to_string()
    // match option_env!("RIPGREP_BUILD_GIT_HASH") {    //ripgrep 这个环境变量是通过 build.rs 设置的
    //     None => semver.to_string(),
    //     Some(hash) => format!("{semver} (rev {hash})"),
    // }
}