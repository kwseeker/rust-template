/// str 类型测试

/// str 有一个 parse() 方法可以将字符串切片转换为其他类型
/// 内部借助 FromStr::from_str(self) 实现
#[test]
fn test_parse() {
    // 简单类型转换
    let four = "4".parse::<u32>().unwrap();
    assert_eq!(4u32, four);

    // 官方示例只展示了简单类型的转换，其实可以通过重写 from_str() 方法实现更复杂的转换，参考 ripgrep printer color.rs 中实现
}