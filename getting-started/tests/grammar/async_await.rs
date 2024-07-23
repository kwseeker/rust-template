/// Rust 异步编程 async/await
/// 参卡资料：推荐 《Rust编程圣经》C4.11 https://course.rs/advance/async/intro.html
/// 程序设计中的并发模型：
///     OS线程
///     事件驱动
///     协程
///     Actor模型
///     async/await
/// Rust async/await 这一套和 Java Reactive Stream 那一套是相同的并发模型。
/// 其实看Java一些异步框架实现，本质还是基于回调，只是将回调封装成了更好用的形式，比如Java Reactor 的发布订阅。
/// Rust async/await 内部实现原理有空验证下是否一样 TODO
/// Rust 并发处理到底应该选择 多线程 还是 async ?
///     有大量 IO 任务需要并发运行时，选 async 模型
///     有部分 IO 任务需要并发运行时，选多线程，如果想要降低线程创建和销毁的开销，可以使用线程池
///     有大量 CPU 密集任务需要并行运行时，例如并行计算，选多线程模型，且让线程数等于或者稍大于 CPU 核心数
///     无所谓时，统一选多线程

use futures::executor::block_on;

/// 执行器执行 Future 任务并阻塞等待执行结束
#[test]
fn blocked() {
    async fn hello_world() {
        println!("hello, rust async!");
    }

    // 下面两行的注释是《Rust语言圣经》的注释，感觉说的不是很容易理解
    // 如果从Java Future实现的角度理解的话：
    // 下面第一行只是定义一个任务，任务定义后返回一个 Future，这个 Future 是一个后门用于在任务执行完成后获取执行结果
    // 下面第二行则是通过执行器（基于线程的包装，比如线程池）执行上面的任务，并等待执行完毕
    // 当然可能 Rust 内部的实现和 Java 完全不同，需要深入源码验证 TODO
    let future = hello_world(); // 返回一个Future, 因此不会打印任何输出
    block_on(future); // 执行`Future`并等待其运行完成，此时"hello, world!"会被打印输出
}

/// 执行器执行带返回值的 Future 任务并阻塞等待执行结束获取返回值
#[test]
fn blocked_with_return_value() {
    async fn current_timestamp() -> u64 {
        // 获取当前时间毫秒数
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    let future = current_timestamp();
    let timestamp = block_on(future);
    println!("timestamp when async task run: {timestamp}")
}

/// 使用 .await 等待异步代码执行完毕
#[test]
fn await_future() {
    async fn current_timestamp() -> u64 {
        // 获取当前时间毫秒数
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    async fn print_timestamp() {
        let timestamp = current_timestamp().await;
        println!("timestamp when async task run: {timestamp}")
    }

    block_on(print_timestamp());
}

/// 使用 .await 等待异步代码执行完毕
/// task1 task2 有先后顺序，task3 和前两个任务无先后顺序， 这里其实就实现了任务编排
/// 对比 Java CompletableFuture 的任务编排写法真的是干净整洁
/// 结果：
///     before tasks
///     exec task1
///     middle tasks
///     exec task3      //task3
///     exec task2
///     after tasks
#[test]
fn await_future2() {
    use tokio::runtime::Runtime;
    use tokio::time::{sleep, Duration};

    async fn task1() {
        // 随机睡眠0-1000ms
        // sleep(Duration::from_millis(rand::random::<u64>() % 1000)).await;
        sleep(Duration::from_millis(1000)).await;
        println!("exec task1")
    }

    async fn task2() {
        // 随机睡眠0-1000ms
        sleep(Duration::from_millis(1000)).await;
        println!("exec task2")
    }

    async fn task3() {
        // 随机睡眠0-1000ms
        sleep(Duration::from_millis(1000)).await;
        println!("exec task3")
    }

    async fn exec_task_1_2() {
        println!("before tasks");
        task1().await;
        println!("middle tasks");
        task2().await;
        println!("after tasks");
    }

    async fn exec_all_tasks() {
        let f1 = exec_task_1_2();
        let f2 = task3();
        futures::join!(f1, f2);
    }

    let rt = Runtime::new().unwrap();
    rt.block_on(exec_all_tasks());
}