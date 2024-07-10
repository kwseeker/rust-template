/// std 中的内部函数
/// saturating_sub： 饱和减法, 如果减法运算的结果超出了数值类型的表示范围，结果会被“饱和”到该类型的最小（或最大）可表示值，而不是产生溢出。

#[test]
fn saturating_sub() {
    let a: u8 = 1;
    let b: u8 = 2;
    let result = a.saturating_sub(b); // 结果为 0 , u8 最小值为0, 1-2=-1，超出了u8可以表示范围，会饱和到最小值
    assert_eq!(result, 0)
}