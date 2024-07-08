use std::any::type_name;

/// 自动派生和手动实现的 Trait 类型区别
/// 自动派生的原理？ TODO 详细实现原理？
///     《Rust编程之道》中提到可以通过过程宏实现自定义派生；
///     《Rust程序设计语言》中列举了一些官方支持的可派生的 trait, 参考：“附录 C：可派生的 trait”

#[test]
fn compare_derive_and_impl() {

    #[derive(Debug)]
    struct Human {
        name: String,
        age: u8,
        weight: f32,
    }

    impl Default for Human {
        fn default() -> Self {
            Human {
                name: "someone".to_string(),
                age: 0,
                weight: 0.0,
            }
        }
    }

    #[derive(Debug, Default)]   // 自动派生 Default trait 会创建一个实例，各个字段设置为字段类型的默认“零”值，可能是编译器配合宏实现的, TODO
    struct Human2 {
        name: String,
        age: u8,
        weight: f32,
    }

    let human1 = Human::default();
    let human2 = Human2::default();
    println!("{human1:?}");
    println!("{human2:?}");
}