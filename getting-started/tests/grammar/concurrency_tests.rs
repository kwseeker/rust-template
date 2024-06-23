//Rust并发编程
//Rust 并发通信模型参考自Golang，也主张 “不要通过共享内存来通讯；而是通过通讯来共享内存。”

use std::alloc::System;
use std::sync::mpsc;
use std::thread;
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