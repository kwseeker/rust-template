use std::fmt::{Display, Formatter};

/// 泛型类型
/// 泛型类型既可以接收引用也可以接收值

/// 泛型参数既可以接收引用也可以接收值
#[test]
fn test_generic() {

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        // 泛型类型既可以接收引用也可以接收值
        fn new(x: T, y: T) -> Point<T> {
            Point {
                x,
                y,
            }
        }
    }

    let xp = &1;
    let yp = &2;
    let point = Point::new(xp, yp);
    let x = 1;
    let y = 2;
    let point2 = Point::new(x, y);
    println!("{point:?}");
    println!("{point2:?}");
}

/// Option<T> 表示一个可有可无的泛型参数
#[test]
fn test_generic_option() {
    fn some_func<T>(p: Option<T>)
    where
        T: Display
    {
        match p {
            Some(v) => println!("{v}"),
            None => println!("None"),
        }
    }

    // some_func(None);    // 无法自动推断出 T 的类型，会报错
    some_func::<String>(None);  //虽然可以这样随便指定一个类型，但是 None 根本就不在乎这个类型是什么，这么写有点奇怪
}

#[test]
fn test_generic_option2() {
    struct Null {
    }
    impl Display for Null {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }

    fn some_func<T>(p: Option<T>)
    where
        T: Display
    {
        match p {
            Some(v) => println!("{v}"),
            None => println!("None"),
        }
    }

    some_func::<Null>(None);    // 无法自动推断出 T 的类型，会报错
}
