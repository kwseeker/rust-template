/// Trait 约束
/// 这里讨论几种常见约束的意义
/// Debug
/// ’a              如： T: 'a,       规定泛型 T 内部的借用引用的生命周期必须长于 'a （意味着该类型不能传递包含生命周期短于 'a 的任何引用）, 'a 是生命周期泛型
/// ‘static         如： T: 'static,  规定泛型 T 内部不包含除 ’static 之外的借用引用
/// 'b: 'a                          规定泛型 'b 生命周期必须长于泛型 'a, TODO
/// ?Sized                          使用一个不定大小的泛型类型，允许你定义一个可以接受动态大小类型的函数或 trait
/// Send 、Sync
/// UnwindSafe 、RefUnwindSafe
/// Default                         TODO

use std::fmt::{Debug, Formatter, Pointer};
use std::thread;

/// ---------------------------------------------------------------------------
/// Debug
#[test]
fn test_trait_constraint_debug() {
    let ver = Version {};
    println!("{:?}", ver);
}

trait Flag: Debug {
    fn long_name(&self) -> String;
}

// #[derive(Debug)]，Flag 上有 Debug 约束，需要自动派生 Debug 或者自定义 Debug 方法实现
struct Version;

impl Flag for Version {
    fn long_name(&self) -> String {
        String::from("version")
    }
}

/// 可以使用 #[derive(Debug)] 自动派生Debug实现代替这个自定义实现
impl Debug for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Version{{long_name:{}}}", self.long_name()))  //{{}} 用于转义
    }
}

/// ---------------------------------------------------------------------------
/// 'static 约束
/// trait Flag: 'static {} 表示 Flag 不能包含除 'static 之外的借用引用
/// trait Flag {}
#[test]
fn test_trait_constraint_static() {
    let v = Version2 { ver: "1.0.0" };
    println!("ver: {:?}", v);
}

trait Flag2: 'static {
    fn long_name(&self) -> String;
}

#[derive(Debug)]
struct Version2<'a> {   //下面约束了 'a 为 'static
    ver: &'a str,
}

impl Flag2 for Version2<'static> {
    fn long_name(&self) -> String {
        String::from("version")
    }
}

/// ---------------------------------------------------------------------------
/// Send Sync 约束
/// Send: 实现 Send 的类型可以安全地在线程间传递所有权，也就是说， 可以跨线程移动。  TODO 这句话怎么理解？内部到底做了什么？
/// Sync: 实现了 Sync 的类型可以安全地在线程间传递不可变借用，也就是说， 可以跨线程共享。   TODO 这句话怎么理解？
/// Send 和 Sync 是自动 trait。一个类型自动实现 Send 如果：
///     它不包含任何不满足 Send 的非 Copy 类型字段。
///     所有字段的类型都实现了 Send。
/// 类似地，一个类型自动实现 Sync 如果：
///     它不包含任何不满足 Sync 的非 Copy 类型字段。
///     所有字段的类型都实现了 Sync。
#[test]
fn test_trait_constraint_send() {
    let data = vec![1, 2, 3];   //Vec 自动实现了 Send Sync
    let handle = thread::spawn(move || {
        // 在新线程中使用 data
        println!("{:?}", data); //可以安全地在多线程之间传送， 不会报错
    });
    handle.join().unwrap();
}

#[test]
fn test_thread_safe() {
    // let mut s = "Hello".to_string();    //String 没有实现 Send Sync
    // for _ in 0..3 {
    //     thread::spawn(move || {         //无法安全地在线程之间传递，这里会报错
    //         s.push_str(" Rust!");
    //     });
    // }
}