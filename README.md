# rust-template
rust基础、技术栈测试。

模块说明：

+ **getting-started**

  Rust 语法入门练手项目（语法入门有项目支撑效果更好），功能是 grep 文本搜索，是开源项目 [ripgrep](https://github.com/BurntSushi/ripgrep) 的简化瘦身版本，代码数据结构和 ripgrep 保持一致，但是剔除了大部分不重要的功能（源码5W行压缩到7K行），只保留了核心功能和常用的选项。

  代码压缩后很适合用于理解 ripgrep 的工作机制，数据结构中字段都加上了注释。

  从语法学习角度，项目可以覆盖 Rust 绝大部分语法。

+ **api-tpl-rs-example**

  基于 [api-tpl-rs](https://rustcc.cn/article?id=3b503d98-8215-4e9c-88d2-255db4bf228c) 脚手架实现的 Web 服务。

+ **encoding-rs-io-example**

  [encoding-rs-io]() 用于字符集编码转换，由于 Rust 默认使用 UTF-8 编码，Rust 读取其他编码数据时需要进行编码转换。

+ **walkdir-example**

  [walkdir]() 提供对工作目录进行处理的实用函数，比如递归遍历。

+ todo ...

