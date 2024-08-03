//生命周期
//Rust中的每一个引用都有其生命周期，即保持有效的作用域。大部分时候生命周期是隐含并可以推断的。
//生命周期的主要目标是避免悬垂引用（dangling references），后者会导致程序引用了非预期引用的数据。
//如何避免悬垂引用（悬垂指针）：
//  Rust使用借用检查器（Borrow checker）检查借用生命周期的正确性，当一个变量的生命周期大于内部引用的另一个变量的生命周期，那么编译器就会报错
//关于生命周期标注：
//  注意：在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过。
//  例如一个变量，只能活一个花括号，那么就算你给它标注一个活全局的生命周期，它还是会在前面的花括号结束处被释放掉，并不会真的全局存活。
//  举个例子：这里函数中定义了一个生命周期约束'a, 还约束了x、y这两个引用的生命周期必须要都不小于'a, 最终会返回x、y生命周期的交集部分
//  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
//  }
//  let string1 = String::from("long string is long");              //假设string1生命周期是 m
//  {
//      let string2 = String::from("xyz");                          //假设string1生命周期是 n，明显 n<m
//      let result = longest(string1.as_str(), string2.as_str());   //result 的生命周期最终是 n
//      println!("The longest string is {}", result);
//  }
//  有些书籍这点没讲明白，很容易会导致对生命周期的误解，以为标注生命周期会改变原本的实际作用域，推荐参考 《Rust语言圣经》https://course.rs/basic/lifetime.html
//生命周期判断规则：
// 第一条规则：编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个
// 引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32) ，有两个引用参数的函数就有两
// 个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32) ，依此类推。
// 第二条规则：如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：
// fn foo<'a>(x: &'a i32) -> &'a i32 。
// 第三条规则：如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self ，
// 说明是个对象的方法 (method)(译者注：这里涉及 rust 的面向对象参见 17 章)，那么所有输出
// 生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。

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