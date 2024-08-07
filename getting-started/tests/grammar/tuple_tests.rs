//Rust 元组测试
//元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
//和数组的区别：
//1 数组是同质的；元组是异构的（即可以包含不同类型的值）。
//2 元组存储在堆heap内存中；数组存储在栈stack内存中。
//3 数组的元素存储在连续的内存位置中。
//4 元组使用括号 () 初始化，而数组使用方括号 [] 初始化。
//5 单元元组（空元组）常用于表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。

#[test]
fn test_tuple() {
    // let tup: (i32, i8, f32) = (100, 2, 3.5);
    let tup = (100, 2, 3.5);
    //解构
    let tup2: (i32, i8, f32) = tup;
    println!("Elements in tup2: {}, {}, {}", tup2.0, tup2.1, tup2.2)
}

#[test]
fn test_unit_tuple() {
    //无返回值的方法、函数、闭包都会隐式返回单元元组
    let closure_a = || {
        println!("exec closure！");
    };
    let ret = closure_a();
    assert_eq!((), ret);
}