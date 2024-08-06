# rust-template
rust基础入门、技术栈测试、项目实战。

> Rust 不太容易学，里面有一些很新的概念，没有参照比较难理解，而且很难找到解释的清楚的资料，中文资料比较少，感觉需要多读标准库源码以及开源组件源码加深理解，搜索问题应该多用英语搜索，学习基础不要迷信 ChatGPT 等工具的解释很可能包含错误信息。

模块说明：

+ **入门**

  + **getting-started**

    Rust 语法入门练手项目（语法入门有项目支撑效果更好），功能是 grep 文本搜索，是开源项目 [ripgrep](https://github.com/BurntSushi/ripgrep) 的简化瘦身版本，代码数据结构和 ripgrep 保持一致，但是剔除了大部分不重要的功能（源码5W行压缩到7K行），只保留了核心功能和常用的选项。

    代码压缩后很适合用于理解 ripgrep 的工作机制，数据结构中字段都加上了注释。

    从语法学习角度，项目可以覆盖 Rust 绝大部分语法。

+ **实战**

  + **ai-code-reviewer**

    一个简单的 Github PR 代码 AI 自动评审工具，基于 Github Actions + Github API + ChatGLM。

  + **rust-glm**

    智谱 AI 大模型 ChatGLM Rust SDK。

+ **技术栈测试**


  + **Web 服务**

    + **api-tpl-rs-example**

      基于 [api-tpl-rs](https://rustcc.cn/article?id=3b503d98-8215-4e9c-88d2-255db4bf228c) 脚手架实现的 Web 服务。


  + **错误处理**
    + **anyhow-example**


  + **命令行解析**
    + **clap-example**
    + **lexopt-example**



  + **字符编码**

    + **encoding-rs-io-example**

      [encoding-rs-io]() 用于字符集编码转换，由于 Rust 默认使用 UTF-8 编码，Rust 读取其他编码数据时需要进行编码转换。

  + **文件系统**


      + **walkdir-example**

        [walkdir]() 提供对工作目录进行处理的实用函数，比如递归遍历。


  + **其他**
    + **lazy-static-example**
    + **rsntp-example**
  + **正则表达式**
    + **regex-example**
  + **HTTP 请求**
    + **reqwest-example**
  + **序列化和反序列化**
    + **serde-json-example**
  + **异步编程**
    + **tokio-example**
  + **配置文件解析**
    + **toml-example**

