//生命周期
//Rust中的每一个引用都有其生命周期，即保持有效的作用域。大部分时候生命周期是隐含并可以推断的。
//生命周期的主要目标是避免悬垂引用（dangling references），后者会导致程序引用了非预期引用的数据。
//生命周期判断规则：
// 第一条规则是编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个
// 引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32) ，有两个引用参数的函数就有两
// 个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32) ，依此类推。
// 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn
// foo<'a>(x: &'a i32) -> &'a i32 。
// 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self ，
// 说明是个对象的方法 (method)(译者注：这里涉及 rust 的面向对象参见 17 章)，那么所有输出
// 生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符
// 号。

// 'a 'b ... 等等称为泛型生命周期，泛型生命周期可以应用于函数、方法、结构体，但是不能用于变量定义
#[test]
fn test_lifecycle() {
    // let r;
    // {
    //     let x= 5;   //代码块中定义的变量，再出代码块后就失效了
    //     r = &x;
    // }
    // println!("r: {r}")  //引用一个失效的值会报错， 悬垂引用
    let r: &'static i32;
    {
        let x: &'static i32 = &5;   //代码块中定义的变量，再出代码块后就失效了
        r = x;
    }
    println!("r: {r}")  //引用一个失效的值会报错
}

#[test]
fn test_lifecycle2() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }

    println!("The longest string is {result}");
}

// 'a 'b ... 等等称为泛型生命周期
// fn longest<'a>(x: &str, y: &str) -> &str {   //这么写会导致返回值生命周期的不确定性
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//包含引用的结构体，需要添加泛型生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[test]
fn test_lc_in_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}