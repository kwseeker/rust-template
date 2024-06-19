// Rust 没有空值，空是通过 Option<T> 的一个枚举值（None）表示的
#[test]
fn option_usage() {
    let some_num = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}