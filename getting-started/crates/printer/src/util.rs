use grep_matcher::{LineTerminator, Match};

/// 从给定切片中修剪前缀 ASCII 空白字符并返回相应的范围。一旦看到非空格或行终止符，就会停止修剪前缀。
/// 比如一个字符串前面有两个空格，会删除这两个空格，但是不是在原字符串上删而返回一个新的Match对象（忽略掉空白字符的范围）
pub(crate) fn trim_ascii_prefix(
    line_term: LineTerminator,
    slice: &[u8],
    range: Match,
) -> Match {
    fn is_space(b: u8) -> bool {
        match b {
            b'\t' | b'\n' | b'\r' | b' ' => true,
            // b'\x0B' 垂直定位符号、 b'\x0C' 换页键
            b'\x0B' | b'\x0C' => {
                println!("检测到 b'\x0B' 垂直定位符号 或 b'\x0C' 换页键");
                true
            }
            _ => false,
        }
    }

    let count = slice[range]
        .iter()
        .take_while(|&&b| -> bool { // 第一个 & 表示我们借用了迭代器中的元素
                                        // 第二个 & 是闭包参数模式的一部分，它解引用了闭包接收到的引用，因此 &&b 实际上得到的是 b 的值 u8
            is_space(b) && !line_term.as_bytes().contains(&b)
        })
        .count();
    range.with_start(range.start() + count)
}