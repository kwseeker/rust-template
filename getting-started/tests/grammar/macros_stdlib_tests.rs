/// 标准库中定义的宏
/// cfg!
/// write!

use std::fmt::Write;    //String 源码明明有实现 std::fmt:Write, 为何 write! 会报错？ error[E0599]: cannot write into `String`
                        //提示了一个关键信息：
                        //help: trait `Write` which provides `write_fmt` is implemented but not in scope; perhaps you want to import it
                        //是说 write_fmt 已经实现了但是没有在当前作用域，需要导入。但是为何不会自动导入？String 明明实现了 std::fmt::Write
#[test]
fn test_write() {
    let mut col1 = String::new();
    write!(col1, r"-h").unwrap();           //本质是调用 write_fmt ，这个方法是默认实现了的
    write!(col1, ",").unwrap();
    write!(col1, r"--help").unwrap();
    println!("{col1}")
}

// #[cfg(not(no_global_oom_handling))]
// #[stable(feature = "rust1", since = "1.0.0")]
// impl fmt::Write for String {
//     #[inline]
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         self.push_str(s);
//         Ok(())
//     }
//
//     #[inline]
//     fn write_char(&mut self, c: char) -> fmt::Result {
//         self.push(c);
//         Ok(())
//     }
// }