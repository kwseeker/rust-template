//所有权测试
//Rust 所有权负责：跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间。
//所有权规则：
// 1. Rust 中的每一个值都有一个 所有者（owner）。
//    这个所有者官方的文档并没有说到底是什么？只有一个模糊的表述，不过感觉应该是栈帧，TODO：参考下其他资料看下所有者到底是什么。
// 2. 值在任一时刻有且只有一个所有者。
// 3. 当所有者（变量）离开作用域，这个值将被丢弃。
//Rust 内存管理策略：
// 内存在拥有它的变量离开作用域后就被自动释放。
//引用 & 借用：
//  创建引用的行为称作借用，引用并不拥有值的所有权。
//  可变引用 & 不可变引用
//  引用规则：
//  • 在任意给定时间，要么只能有一个可变引用（一个作用域同时只能有一个可变引用），要么 只能有多个不可变引用。
//  • 引用必须总是有效的。
// 方法参数 &self 与 self：
//  &self 类型方法可以接收 拥有实例、实例的引用 的调用，拥有实例其实是被隐式地转成了实例的引用
//  self 类型方法只接收 拥有实例 的调用

#[test]
fn test_move() {
    let s1 = String::from("hello");
    // let s2 = s1;                 //移动操作：将“hello”字符串的引用传递给s2的同时，将s1给释放掉了
    // println!("{s1}, world!");    //再次访问s1由于已经被释放了会报错
    //如果想让s1仍然可用应该使用克隆操作
    let s2 = s1.clone();     //s1 和 s2 指向两块不同的内存
    println!("{s1}, world!");
    println!("{s2}, world!");

    let x = 5;
    let y = x;                 //像这种基本数据类型，赋值操作本质是值拷贝
    println!("x = {x}, y = {y}");
}

#[test]
fn test_macro_move() {
    let str = String::from("hello");
    // let func = |str: String| {  //这里定义一个闭包，||中定义参数
    //     println!("str: {str}")
    // };
    // func(str);
    let func = || {     //闭包可以自动捕获上下文环境，并不需要像上面那样传递参数
        println!("str: {str}")
    };
    func();

    let mut str2 = String::from("hello");
    let func = || {
    // let func = move || {     //使用 move 移动所有权
        println!("str2: {str2}")
    };
    func();
    println!("str2: {str2}")    // 没有 move 时可以访问，使用 move 移动所有权后不可以访问（会在闭包执行结束后清理掉内存）
}

#[test]
fn test_mut_ref() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    // println!("Between defining and calling closure: {list:?}");  //Rust规定有可变借用存在时不允许有其他的借用
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

#[test]
fn test_deref() {
    // len() 是 String的方法，s是String的不可变引用，这里为何不需要解引用
    fn calculate_length(s: &String) -> usize {
        // (*s).len()   // 先解引用再调用也是可以的
        s.len()
    }

    let str = String::from("abc");
    let len = calculate_length(&str);
    println!("len: {len}")
}

/// &self 类型方法可以接收 拥有实例、实例的引用 的调用，拥有实例其实是被隐式地转成了实例的引用
/// self 类型方法只接收 拥有实例 的调用
#[test]
fn self_ref() {
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {         //支持通过引用调用这个方法
            self.width * self.height
        }

        fn area2(self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {     //rect1是拥有（owning）实例
        width: 30,
        height: 50,
    };
    let rect2 = &rect1; //rect2是rect1的引用
    println!("The area of the rectangle is {} square pixels.", rect1.area());    //rect1既可以调用 area(&self) （内部将rect1隐式转换为了&rect1） 也可以调用 area2(self)
    println!("The area of the rectangle is {} square pixels.", rect1.area2());
    // println!("The area of the rectangle is {} square pixels.", rect2.area2());   //不能通过引用调用 area2(self)
}
