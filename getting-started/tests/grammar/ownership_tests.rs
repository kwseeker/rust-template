//所有权测试
//Rust 所有权负责：跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间。
//所有权规则：
// 1. Rust 中的每一个值都有一个 所有者（owner）。
//    这个所有者官方的文档并没有说到底是什么？只有一个模糊的表述，不过感觉应该是栈帧，TODO：参考下其他资料看下所有者到底是什么。
// 2. 值在任一时刻有且只有一个所有者。
// 3. 当所有者（变量）离开作用域，这个值将被丢弃。
//Rust 内存管理策略：
// 内存在拥有它的变量离开作用域后就被自动释放。

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

//TODO Copy trait & Drop trait
