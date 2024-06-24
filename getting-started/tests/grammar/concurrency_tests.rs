//Rust并发编程
//Rust 并发通信模型参考自Golang，也主张 “不要通过共享内存来通讯；而是通过通讯来共享内存。”
//不过个人认为通信最终也是通过共享内存实现的，只不过实现流程上的区别，就好比响应式本质也是回调，但是却没有回调地狱的问题
//通信方式：
// mpsc::channel(); 这种方式类似于Java中被大量使用的生产者消费者模式：一些线程将数据发到队列，有一个线程从队列读取数据进行处理。
//共享内存方式：
// 互斥器
//Send 与 Sync Trait （Rust 默认线程安全）
// Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间传送。几乎所有的 Rust 类型都是 Send 的。
// Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。换一
// 种方式来说，对于任意类型 T ，如果 &T （T 的不可变引用）是 Send 的话 T 就是 Sync 的，
// 这意味着其引用就可以安全的发送到另一个线程。
// 任何完全由 Send 的类型组成的类型也会自动被标记为 Send 。几乎所有基本类型都是 Send的，除了裸指针（raw pointer）
//无惧并发编程：
// 借助 Send 和 Sync，再配合所有权机制，带来的效果就是， Rust 能够在编译期就检查出数据竞争的隐患，而不需要等到运行时再排查。

use std::alloc::System;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::{JoinHandle, spawn};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[test]
fn thread_create() {
    //vec! 是一个宏用于根据提供的值便捷地创建一个新的 Vec 向量， TODO 如何自定义宏
    let v = vec![1, 2, 3];
    //创建线程
    //spawn 传递一个闭包（对应泛型参数F，返回值对应泛型参数T，如果无返回值会返回单元元组）
    // let handle = thread::spawn(|| {      //不使用 move 编译时会报错，因为无法保证闭包执行时 v 的有效性，比如刚创建好线程还没执行，在另一个线程就将 v 释放掉
    let handle = thread::spawn(move || {
        println!("Here‘s a vector： {v:?}");
        return v.len();      //如果闭包没有返回值，实际会返回一个空元组
    });
    //等待线程结束，join() 返回的是标准库中的 Result 枚举值，包装线程闭包中返回的值
    let ret = handle.join().unwrap();
    println!("thread returned: {ret}")
}

//通过信道通信
#[test]
fn thread_communication() {
    let (tx, rx) = mpsc::channel();

    //新线程发
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    //主线程收
    let received = rx.recv().unwrap();  //阻塞的方式读取
    println!("Got: {received}");
}

#[test]
fn thread_communication2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // tx.send(val).unwrap();       //val 发给另一个线程，而后面还读取 val， 可能另一个线程在读取 val 前修改或丢弃了 val，这是不安全的，rust编译器不允许这样的行为
        tx.send(val.clone()).unwrap();  //修复方法：克隆一个新的值传过去
        println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

//轮询发送多条消息
#[test]
fn thread_communication3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            current_timestamp();
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {        //Receiver 内部实现了迭代器 Trait，所以这里可以直接迭代读取接收到的内容
        println!("Got: {received}");
    }
}

fn current_timestamp() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            println!("send message at {}", n.as_millis())
        }
        Err(err) => {
            println!("error occurred: {}", err)
        }
    }
}

//克隆多个生产者发送消息到一个消费者
#[test]
fn thread_communication4() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    thread::spawn(move || {
        tx.send("message-1").unwrap();
    });
    thread::spawn(move || {
        tx1.send("message-2").unwrap();
    });
    thread::spawn(move || {
        tx2.send("message-3").unwrap();
    });

    for received in rx {
        println!("Got: {received}");
    }
}

//互斥器 Mutex
//从官方注释看，lock() 是阻塞等待获取锁，如果别的获取锁的线程出现panic，则在获取互斥锁后，此调用将返回错误，且不支持重入
//锁支持自动释放
#[test]
fn thread_mutex() {
    //基于其他语言猜测，Rust Mutex 可能也是基于比较交换的方式实现的, 这里的传参是被保护的数据
    let m = Mutex::new(5);
    println!("m = {m:?}");
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        // num = m.lock().unwrap();    //不支持重入
    }   //自动解锁
    println!("m = {m:?}");
    {
        let mut num = m.lock().unwrap();
        println!("num = {num:?}");
    }
    println!("m = {m:?}");
}

#[test]
fn thread_mutex2() {
    let balance = 100;
    //下面两个线程操作的余额和上面balance完全是不同的值
    let m = Arc::new(Mutex::new(balance));  //balance值传递

    let m1 = Arc::clone(&m);    //这里克隆实现多所有权而不是重新深克隆了一个值，不同线程是可以读取彼此修改后的最新值的
    let h1 = thread::spawn(move || {
        let mut balance = m1.lock().unwrap();
        *balance += 10;
        println!("thread1 exec: num = {balance:?}");
        thread::sleep(Duration::from_millis(100));
        println!("thread1 done");
    });

    let m2 = Arc::clone(&m);
    let h2 = thread::spawn(move || {
        let mut balance = m2.lock().unwrap();
        *balance -= 20;
        println!("thread2 exec: num = {balance:?}");
        thread::sleep(Duration::from_millis(100));
        println!("thread2 done");
    });

    h1.join().unwrap();
    h2.join().unwrap();
    println!("balance = {balance}");    //100
}

// 账户扣款，互斥锁方式
#[test]
fn thread_deduct_balance() {
    let balance = Mutex::new(100);
    let arc_balance = Arc::new(balance);
    let mut thread_vec = vec![];
    //开20个线程扣款，每个线程扣10元
    for i in 0..20 {
        let arc_balance = Arc::clone(&arc_balance);
        let handle = thread::spawn(move || {
            let mut balance = arc_balance.lock().unwrap();
            //稍微延迟一下确保各个线程发生竞争
            thread::sleep(Duration::from_millis(50));
            if *balance >= 10 {
                *balance -= 10;
            } else {
                println!("balance not enough!")
            }
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    let final_balance = arc_balance.lock().unwrap();
    assert_eq!(0, *final_balance);
    println!("final balance: {final_balance}")
}

// 账户扣款，通信方式
#[derive(Debug)]
struct DeductMessage {
    tx: Sender<DeductResult>,
    deduct: u32,
}

#[derive(Debug)]
enum DeductCode {
    SUCCESS {code: String},
    FAILED {code: String, reason: String}
}

struct DeductResult {
    code: DeductCode,
    balance: Option<u32>,
}

#[test]
fn thread_deduct_balance_by_channel() {
    let mut balance = 100u32;
    let (tx, rx) = mpsc::channel();
    let mut thread_vec = vec![];

    //开20个线程扣款，每个线程扣10元
    for _ in 0..20 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let (cb_tx, cb_rx) = mpsc::channel();
            //线程发消息说要扣款，实际扣款在主线程中处理
            tx.send(DeductMessage{tx: cb_tx, deduct:  10u32}).unwrap();
            let deduct_result = cb_rx.recv().unwrap();
            match deduct_result.code {
                DeductCode::SUCCESS {..} => {
                    println!("deduct succeed: balance={}", deduct_result.balance.unwrap());
                }
                DeductCode::FAILED {reason, .. } => {
                    println!("deduct failed: reason={}", reason);
                }
            }

        });
        thread_vec.push(handle);
    }

    let handle = thread::spawn(move || {
        //接收消息，执行真正的扣款
        let mut counter = 0;
        for _ in 0..20 {
            let received = rx.recv().unwrap();
            println!("received deduct: {}", received.deduct);
            if balance >= received.deduct {
                balance -= received.deduct;
                println!("deduct: {}, {balance}", received.deduct);
                let deduct_result = DeductResult{code: DeductCode::SUCCESS {code: String::from("0")}, balance: Some(balance)};
                received.tx.send(deduct_result).unwrap();
            } else {
                println!("balance not enough!");
                let deduct_result = DeductResult{code: DeductCode::FAILED {
                    code: String::from("1"), reason: String::from("balance not enough!")}, balance: None};
                received.tx.send(deduct_result).unwrap();
            }
            counter += 1;
        }
    });
    thread_vec.push(handle);

    for handle in thread_vec {
        handle.join().unwrap();
    }
}

// 掷硬币实验
#[test]
fn thread_deadlock() {
    // 所有实验总共掷硬币次数
    let total_flips = Arc::new(Mutex::new(0));  //从0开始计数， Mutex实现对计数的并发安全保护，Arc则用于实现多所有权并在各线程间共享
    // 实验完成线程数量
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_flips = 10;

    // 开8个线程做投掷硬币实验，每个实验要求连续出现10次正面即结束
    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {                         //因为 Mutex 没有实现 Copy 特征
            flip_simulate(target_flips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }
    loop {
        let completed = completed.lock().unwrap();
        if *completed == runs {
            let total_flips = total_flips.lock().unwrap();
            println!("Final average: {}", *total_flips / *completed);
            break;
        }
    }
}

fn flip_simulate(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
    let mut continue_positive = 0;
    let mut iter_counts = 0;
    while continue_positive <= target_flips {
        iter_counts += 1;
        let pro_or_con = rand::random();
        if pro_or_con {
            continue_positive += 1;
        } else {
            continue_positive = 0;
        }
    }
    println!("iter_counts: {}", iter_counts);
    let mut total_flips = total_flips.lock().unwrap();
    *total_flips += iter_counts;
}
