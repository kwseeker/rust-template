// Trait 定义共同行为
// 定位和其他语言的接口比较类似， trait：意为特征。

// ------------------------------------------------------------------
pub trait Fly {
    fn fly(&self);
}

pub trait Work {
    fn work(&self);
    fn do_something();
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
    fn do_something() {
        println!("do something ...");
    }
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

#[test]
fn test_trait() {
    // let human = Human{};
    let human = Human{ 0: () };
    // let bird = Bird{};
    let bird = Bird{ 0: () };
    human.eat();
    human.drink();
    human.work();
    bird.eat();
    bird.drink();
    bird.fly();
}

// ------------------------------------------------------------------
// Trait 派生
// 调试中常派生 Debug Trait
#[derive(Debug)]    //相当于
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