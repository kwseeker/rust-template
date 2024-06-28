/// Rust 中的各种指针
/// 引用（Reference）、原生指针（Raw Pointer）、函数指针（fn Pointer）、智能指针。
/// 智能指针：
///     Box<T>  Box<T>是指向类型为 T 的堆内存分配值的智能指针。当 Box<T>超出作用域范围时，将调用
///             其析构函数，销毁内部对象，并自动释放堆中的内存。可以通过解引用操作符(*)来获取 Box<T> 中的 T

#[test]
fn test_box() {
    #[derive(PartialEq, Debug)] //TODO PartialEq
    struct Point {
        x: f64,
        y: f64,
    }
    let box_point = Box::new(Point { x: 0.0, y: 0.0 });
    let unboxed_point: Point = *box_point;
    assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
}