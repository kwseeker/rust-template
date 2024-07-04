/// 结构化编程之结构体
/// 结构体的类型：
///     带有字段的结构体
///     元组结构体：用于给元组取一个名字并使其称为不同的类型
///     类单元结构体：没有任何字段的结构体，常用于想要在某个类型上实现 trait 但不需要在类型中存储数据，比如： struct AlwaysEqual;
#[test]
fn test_tuple_structure() {
    // 两个元组具有相同的结构但是表示不同的含义
    struct Color(i32, i32, i32);    //R G B
    struct Point(i32, i32, i32);    //x y z

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}