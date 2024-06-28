/// Rust 模式匹配
/// 参考：官方的《Rust程序设计语言》C18.模式与模式匹配
/// Rust 模式很强大
/// 模式语法：
///     匹配字面值
///     匹配命名变量
///     多个模式
///     通过 ..= 匹配值的范围
///     结构并分解值
///     解构结构体
///     解构枚举
///     解构嵌套的结构体和枚举
///     解构结构体和元组
///     忽略模式中的值（_）
///     用..忽略剩余值
///     匹配守卫提供的额外条件
///     @绑定

#[test]
fn test_match_pattern() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),    //Some(y) 会匹配通过x传进来的任何Some值
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");
}

#[test]
fn test_multi_pattern() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

#[test]
fn test_section_pattern() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let ch = 'b';
    match ch {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

#[test]
fn test_deconstruct() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    //带有字面值匹配的部分解构
    let p = Point { x: 0, y: 7 };
    match p {
        //y必须为0
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        // Point { x: 0, y: 1..=10 } => println!("On the y axis and y between [1,10] at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

#[test]
fn test_nested_deconstruct() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }_
        => (),
    }
}