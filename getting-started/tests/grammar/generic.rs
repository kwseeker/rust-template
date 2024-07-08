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