use std::borrow::Cow;
use std::path::Path;
use bstr::ByteVec;
use grep_matcher::{LineTerminator, Match};

#[derive(Clone, Debug)]
pub(crate) struct PrinterPath<'a> {
    // On Unix, we can re-materialize a `Path` from our `Cow<'a, [u8]>` with
    // zero cost, so there's no point in storing it. At time of writing,
    // OsStr::as_os_str_bytes (and its corresponding constructor) are not
    // stable yet. Those would let us achieve the same end portably. (As long
    // as we keep our UTF-8 requirement on Windows.)
    // #[cfg(not(unix))]
    // path: &'a Path,
    bytes: Cow<'a, [u8]>,
    // ripgrep 还支持超链接路径，这里暂时忽略
    // hyperlink: OnceCell<Option<HyperlinkPath>>,
}

impl<'a> PrinterPath<'a> {
    pub(crate) fn new(path: &'a Path) -> PrinterPath<'a> {
        PrinterPath {
            // #[cfg(not(unix))]
            // path,
            // N.B. This is zero-cost on Unix and requires at least a UTF-8
            // check on Windows. This doesn't allocate on Windows unless the
            // path is invalid UTF-8 (which is exceptionally rare).
            bytes: Vec::from_path_lossy(path),  //TODO bstr 拓展方法
        }
    }

    /// 这个是为了兼容类Unix系统和Windows不同的的路径分割符号，这里不需要
    pub(crate) fn with_separator(mut self, sep: Option<u8>) -> PrinterPath<'a> {
        self
    }
}

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