# Rust入门

从整体到局部地入门Rust编程！

先抛出几个问题：

1. Rust产品级项目结构都包含哪些部分？
2. 上面部分中的代码是怎么被联系起来成为一个整体并被编译、打包以及执行的？
3. Rust如何单元测试？如何调试？

4. Rust项目持续集成（CI）方案？

> 这里通过分析 ripgrep 这个开源项目开发流程，学习Rust项目开发、运维流程和Rust语法。
>
> 从提交历史中选择几个关键的提交节点，分析Rust项目一步步是怎么开发的。
>
> 详细参考 kwseeker/ripgrep。
>
> ripgrep 引入的依赖：
>
> - **aho-corasick**: 用于高效多模式字符串搜索的库，基于 Aho-Corasick 算法。
> - **anyhow**: 提供一个简单的错误处理方式，允许你返回一个通用的错误类型。
> - **bstr**: 提供对字节字符串（byte string）的高级操作，特别是那些可能包含非 UTF-8 数据的字符串。
> - **cc**: 一个构建依赖项，用于编译 C/C++ 代码。
> - **cfg-if**: 用于基于编译时配置条件编译代码。
> - **crossbeam-channel**: 一个用于线程间通信的多生产者，单消费者（MPSC）通道库。
> - **crossbeam-deque**: 提供一个并发安全的双端队列。
> - **crossbeam-epoch**: 用于内存重用和垃圾收集的并发数据结构。
> - **crossbeam-utils**: 包含一些用于构建并发 Rust 程序的工具。
> - **encoding_rs**: 提供对不同字符编码的支持，特别是对非 UTF-8 编码。
> - **encoding_rs_io**: 基于 `encoding_rs` 的 I/O 扩展。
> - **glob**: 用于匹配文件路径的 glob 模式。
> - **itoa**: 用于将整数转换为字符串的快速库。
> - **jobserver**: 用于协调多个编译器进程，以避免编译冲突。
> - **lexopt**: 用于解析命令行选项的库。
> - **libc**: 提供对 C 标准库的 Rust 绑定。
> - **log**: 一个灵活的日志记录接口。
> - **memchr**: 提供快速的内存搜索功能。
> - **memmap2**: 用于内存映射文件的库。
> - **once_cell**: 用于初始化全局状态的线程安全单例模式。
> - **pcre2**: Perl 兼容正则表达式的 Rust 绑定。
> - **pcre2-sys**: `pcre2` 的系统库绑定。
> - **pkg-config**: 用于查询编译器和链接器参数的包。
> - **proc-macro2**: 用于编写 Rust 过程宏的库。
> - **quote**: 用于将 Rust 语法树结构转换为 TokenStream。
> - **regex**: 提供正则表达式引擎。
> - **regex-automata**: 用于构建正则表达式自动机。
> - **regex-syntax**: 用于解析正则表达式语法。
> - **ryu**: 用于快速、正确的浮点数到字符串的转换。
> - **same-file**: 用于检查两个文件路径是否指向同一个文件。
> - **serde**: 一个用于序列化和反序列化数据的框架。
> - **serde_derive**: `serde` 的派生宏，用于自动实现序列化和反序列化。
> - **serde_json**: 提供 JSON 序列化和反序列化功能。
> - **syn**: 用于解析 Rust 代码的库，常用于过程宏。
> - **termcolor**: 用于在终端中输出彩色文本。
> - **textwrap**: 提供文本包装和折行功能。
> - **unicode-ident**: 用于处理 Unicode 标识符。
> - **walkdir**: 用于递归遍历目录。



## Rust项目结构

可以参考参考资料中的模块化编程部分。

 找了一些开源项目基本都包括：

+ Cargo.toml

  根目录下的包管理配置文件。

+ .cargo/  (非必需)

+ crates/

  + `<moduleName>`/
    + Cargo.toml
    + examples/ (非必需)
    + src/
    + tests/

+ tests/

开发流程：

1. cargo new 创建项目； 

2. 修改Cargo.toml，以及实现业务代码；

   main() 方法是程序入口。

3. 使用 cargo build 构建项目；

4. 使用 cargo run 运行项目；

5. 使用 cargo build --release 构建要发布的可执行文件。



## Rust 编译、运行原理

参考：[图解 Rust 编译过程](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rustc_part1.html#图解-rust-编译过程)。

暂没有找到详细资料。

这里先看怎么引入依赖，模块间怎么依赖，Rust面向对象实现等。

Rust 通过 crate（可以理解为项目）、mod（可以理解为命名空间） 管理。通过 use 可以引入其他模块。

Rust 项目默认会引入 Rust 标准库 stdlib，外部依赖则通过 Cargo 引入，可以通过 cargo tree 命令查看项目依赖都是怎么引入的。cargo.lock 中可以查看引入依赖的具体版本号。







## Rust 单元测试与调试

