/// Tokio
/// Runtime 是异步运行时环境，像是一个线程池 + 调度器。
/// 提交异步任务的几种方式：
///     Runtime block_on()
///     Runtime spawn()
///     tokio::spawn()
///     Handle::spawn()     Handle 其实是 Runtime 的一个引用
/// 异步方法中调用其他异步方法，被调用的异步方法默认会被当作新任务提交
/// 两种线程：
///     用于异步任务的工作线程(worker thread)
///     用于同步任务的阻塞线程(blocking thread)，既像独立线程又和runtime绑定，它不受tokio的调度系统调度，tokio不会把其它任务放进该线程，也不会把该线程内的任务转移到其它线程。
/// Tokio 推荐将同步任务交给 blocking thread 执行。
/// Rust 的每个原生线程（std::thread）都对应一个操作系统线程。
/// 绿色线程（green-threads）是用户空间的线程，由程序自身提供的调度器负责调度，由于不涉及系统调用，
/// 同一个OS线程内的多个绿色线程之间的上下文切换的开销非常小，因此非常的轻量级。可以认为，它们就是一种特殊的协程。
///
use std::thread;
use std::time::{Duration, SystemTime};
use tokio::runtime::Runtime;
use tokio::time;

mod task;

/// Runtime 可以理解为是一个线程池 + 调度器
#[test]
fn create_runtime() {
    Runtime::new().unwrap();   // 这种方式创建的 Runtime 是多线程的
    thread::sleep(Duration::from_secs(100));
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// 提交任务并阻塞、在异步方法中新建异步任务
/// 感觉就像是 Java 线程池执行某个任务时，任务内部又往线程池提交了一个新任务，新任务也不会阻塞外部任务的继续执行
/// tokio::spawn() 像 Java 线程池的 submit() 方法
/// rt1.block_on() 像 submit() + future.get()
#[test]
fn nested_async_func() {
    // create an async task start:  1721840150115
    // create an async task end:    1721840150115   ~+0s
    // main thread waiting at:      1721840155117   ~+5s
    // async task over:             1721840160117   ~+10s
    fn async_task() {
        println!("create an async task start: {}", now());
        tokio::spawn(async {
            time::sleep(time::Duration::from_secs(10)).await;
            println!("async task over: {}", now());
        });
        println!("create an async task end: {}", now());    //end 和 start 时间基本相同，可见 tokio::spawn 提交的任务不会阻塞当前线程
    }

    let rt1 = Runtime::new().unwrap();
    // tokio 的 block_on() 提交任务并阻塞当前线程直到提交的任务执行完毕
    rt1.block_on(async {
        // 调用函数，该函数内创建了一个异步任务，将在当前runtime内执行
        async_task();
        time::sleep(time::Duration::from_secs(5)).await;
    });

    // 主线程睡眠 12 秒，确保异步任务执行完毕
    println!("main thread waiting at: {}", now());
    thread::sleep(Duration::from_secs(12));
}

/// 非阻塞提交任务
// main thread waiting at:  1721875644626
// task2 sleep over:        1721875648627
// task1 sleep over:        1721875649627
#[test]
fn runtime_enter() {
    let rt = Runtime::new().unwrap();
    // 进入runtime，但不阻塞当前线程
    let guard1 = rt.enter();    // 这种写法有点像锁，进入runtime上下文

    // 生成的异步任务将放入当前的runtime上下文中执行
    tokio::spawn(async {
        time::sleep(time::Duration::from_secs(5)).await;
        println!("task1 sleep over: {}", now());
    });

    // 释放runtime上下文，这并不会删除runtime
    drop(guard1);                          // 这种写法有点像锁，离开runtime上下文

    // 可以再次进入runtime
    let guard2 = rt.enter();

    tokio::spawn(async {
        time::sleep(time::Duration::from_secs(4)).await;
        println!("task2 sleep over: {}", now());
    });

    drop(guard2);

    // 阻塞当前线程，等待异步任务的完成
    println!("main thread waiting at: {}", now());
    thread::sleep(Duration::from_secs(10));
}

// before spawn task at:    1721880724836
// task1 sleep over:        1721880727838   +3s
// task2 sleep over:        1721880728838   +4s
// task3 sleep over:        1721880729837   +5s
// main thread waiting at:  1721880729837
#[test]
fn runtime_handle() {
    let rt = Runtime::new().unwrap();
    let handle = rt.handle();

    println!("before spawn task at: {}", now());

    handle.spawn(async {
        time::sleep(Duration::from_secs(3)).await;
        println!("task1 sleep over: {}", now());
    });

    let guard = handle.enter();

    tokio::spawn(async {
        time::sleep(time::Duration::from_secs(4)).await;
        println!("task2 sleep over: {}", now());
    });

    drop(guard);

    // block_on 会阻塞当前线程，放到最后
    handle.block_on(async {
        time::sleep(time::Duration::from_secs(5)).await;
        println!("task3 sleep over: {}", now());
    });

    println!("main thread waiting at: {}", now());
    thread::sleep(Duration::from_secs(10));
}

/// 阻塞线程
/// blocking thread不用于执行异步任务，因此runtime不会去调度管理这类线程，
/// 它们在本质上相当于一个独立的thread::spawn()创建的线程，它也不会像block_on()一样会阻塞当前线程。
/// 它和独立线程的唯一区别，是blocking thread是在runtime内的，可以在runtime内对它们使用一些异步操作，例如await。
#[test]
fn blocking_thread() {
}

