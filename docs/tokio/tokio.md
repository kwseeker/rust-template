# [Tokio](https://tokio.rs/tokio/tutorial)

Tokio 是 Rust 语言的一个异步运行时，专为构建高并发网络应用程序设计。
它提供了一种事件驱动的异步 I/O 模型，利用 Rust 的异步 (async) 和等待 (await) 关键字来编写非阻塞的代码。

Tokio提供了几个主要组件:

+ 用于处理异步任务的工具，包括 [synchronization primitives and channels](https://docs.rs/tokio/latest/tokio/sync/index.html) 和 [timeouts, sleeps, and intervals](https://docs.rs/tokio/latest/tokio/time/index.html).。
+ 用于执行异步I/O的API，包括 [TCP and UDP](https://docs.rs/tokio/latest/tokio/net/index.html) sockets, [filesystem](https://docs.rs/tokio/latest/tokio/fs/index.html) 操作, [process](https://docs.rs/tokio/latest/tokio/process/index.html) 和 [signal](https://docs.rs/tokio/latest/tokio/signal/index.html) 管理。
+ 执行异步代码的运行时，包括任务调度程序，由操作系统事件队列（EPOLL，KQUEUE，IOCP等）支持的I/O驱动程序和高性能计时器。

> 感觉一些定位和 Java 生态中的 Vert.x、Netty-Reactor 差不多，不过更宽。





