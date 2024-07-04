/// AsRef trait 用于将值转换为不可变引用，对应的还有一个 AsMut trait 用于将值转换为可变引用
/// pub trait AsRef<T: ?Sized>: Sized {
///     fn as_ref(&self) -> &T; //为某个类型指定as_ref方法，返回一个对该类型的不可变引用
/// }

#[test]
fn test_as_ref() {
    let my_string = "Hello, world!".to_string();
    // String 有实现 AsRef trait，看实现是直接将 self 指针返回了
    let string_ref: &str = my_string.as_ref();
    // 测试发现确实可以直接将 String 的指针赋值给 &str,
    // 因为 String 还实现了 ops::Deref trait，编译器在这里会自动应用  "deref coercion"，即自动将 &String 转换为 &str，
    // 详细参考所有权和借用规则的自动转换，详细参考 《Rust程序设计语言》 C15.2 使用Deref Trait将智能指针当作常规引用处理 函数和方法的隐式 Deref 强制转换
    let string_ref2: &str = &my_string;
    println!("{string_ref2}");
}

