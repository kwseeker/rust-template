// Trait 定义共同行为
// 定位和其他语言的接口比较类似， trait：意为特征。
// 可以说 trait 是 Rust 的灵魂。 Rust 中所有的抽象，比如接口抽象、 OOP 范式抽象、函数式范式抽象等，
// 均基于 trait 来完成。同时， trait 也保证了这些抽象几乎都是运行时零开销的。
// trait 有如下 4 种用法：
//  接口抽象。接口是对类型行为的统一约束。（impl <Trait> for <T>）
//  泛型约束。泛型的行为被 trait 限定在更有限的范围内。
//  抽象类型。在运行时作为一种间接的抽象类型去使用，动态地分发给具体的类型。
//  标签 trait。对类型的约束，可以直接作为一种“标签”使用。
// 孤儿规则：
//  如果要实现某个 trait，那么该 trait 和要实现该 trait 的那个类型至少有一个要在当前 crate 中定义。
// trait 继承：
//  子 trait 可以继承父 trait 中定义或实现的方法。但是不支持传统面向对象的类型继承
// trait 限定（Bound）
// where 关键字：
//  如果为泛型增加比较多的 trait 限定，代码可能会变得不太易读，比如下面这种写法：
//  fn foo<T: A, K: B+C, R: D>(a: T, b: K, c: R){. . .}
//  Rust 提供了 where 关键字，用来对这种情况进行重构：
//  fn foo<T, K, R>(a: T, b: K, c: R) where T: A, K: B+C, R: D {. . .}

// ------------------------------------------------------------------
pub trait Fly {
    fn fly(&self);
}

pub trait Work {
    fn work(&self);
    // fn do_something();
}

pub trait Live {
    fn eat(&self);
    fn drink(&self);
}

//空结构体 与 空元组结构体 区别 ？TODO
// pub struct Human {}
pub struct Human(());

// pub struct Bird {}
pub struct Bird(());

impl Live for Human {
    fn eat(&self) {
        println!("人类生存需要吃食物");
    }

    fn drink(&self) {
        println!("人类生存需要喝水");
    }
}

impl Work for Human {
    fn work(&self) {
        println!("人类需要工作换取生活物资");
    }

    //这是一个函数，不是方法
    // fn do_something() {
    //     println!("do something ...");
    // }
}

impl Live for Bird {
    fn eat(&self) {
        println!("鸟类生存需要吃食物");
    }

    fn drink(&self) {
        println!("鸟类生存需要喝水");
    }
}

impl Fly for Bird {
    fn fly(&self) {
        println!("鸟类基本都会飞");
    }
}

//dyn Work 表示只需要传递一个实现了 Work Trait 的实例，不需要实际类型为 Work
fn dyn_usage(worker: &'static dyn Work) {
    worker.work()
}

#[test]
fn test_trait() {
    // let human = Human{};
    let human = Human { 0: () };
    // let bird = Bird{};
    let bird = Bird { 0: () };
    human.eat();
    human.drink();
    human.work();
    // Human::do_something();
    bird.eat();
    bird.drink();
    bird.fly();
}


#[test]
fn test_dyn_trait() {
    let human: &'static Human = &Human { 0: () };
    //动态派发
    dyn_usage(human);
}


// ------------------------------------------------------------------
// Trait 派生
// 调试中常派生 Debug Trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
fn test_derive() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {rect1:?}"); //在 {} 中加入 :? 指示符告诉 println! 我们想要使用叫做 Debug 的输出格式
    println!("rect1 is {rect1:#?}");
    // println!("rect1 is {rect1}"); //会报错
}

// ------------------------------------------------------------------
// Trait 继承
trait Page {
    fn set_page(&self, p: i32) {
        println!("Page Default: 1");
    }
}

trait PerPage {
    fn set_per_page(&self, num: i32) {
        println!("Per Page Default: 10");
    }
}

struct MyPaginate {
    page: i32,
}

impl Page for MyPaginate {}

impl PerPage for MyPaginate {}

// Paginate 继承 Page PerPage 特征
trait Paginate: Page + PerPage {
    fn set_skip_page(&self, num: i32) {
        println!("Skip Page : {:?}", num);
    }
}

//通过泛型为所有同时拥有 Page 和 PerPage 行为的类型实现 Paginate
//impl<T: A + B> C for T 即 为所有 T⊂(A∩ B)实现 Trait C
impl<T: Page + PerPage> Paginate for T {}

#[test]
fn test_trait_inherit() {
    let my_paginate = MyPaginate { page: 1 };
    my_paginate.set_page(2);
    my_paginate.set_per_page(100);
    my_paginate.set_skip_page(3);
}