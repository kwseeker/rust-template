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

