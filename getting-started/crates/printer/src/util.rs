use std::borrow::Cow;
use std::io;
use std::path::Path;
use bstr::ByteVec;
use termcolor::WriteColor;
use grep_matcher::{LineTerminator, Match, Matcher};
use grep_searcher::{Searcher, SinkError, SinkMatch};

#[derive(Debug)]
pub(crate) struct Sunk<'a> {
    /// 匹配行的字节数组，ripgrep 中这个字段可能经过  Replacer 替换
    bytes: &'a [u8],
    /// 本次搜索缓冲相对于程序起始搜索的绝对偏移，即之前搜索过的数据字节数的累积统计
    absolute_byte_offset: u64,
    /// 缓冲中匹配行的数量
    line_number: Option<u64>,
    // context_kind: Option<&'a SinkContextKind>,
    /// 这个字段 ripgrep 用于记录通过 Replacer 替换之后匹配行在缓冲中的范围，缓冲中可能有多个匹配行所以是个数组
    /// 这里还保持和 original_matches 一致即可
    matches: &'a [Match],
    /// 原始匹配行在缓冲中的范围，缓冲中可能有多个匹配行所以是个数组
    original_matches: &'a [Match],
}

impl<'a> Sunk<'a> {
    #[inline]
    pub(crate) fn empty() -> Sunk<'static> {
        Sunk {
            bytes: &[],
            absolute_byte_offset: 0,
            line_number: None,
            // context_kind: None,
            matches: &[],
            original_matches: &[],
        }
    }

    #[inline]
    pub(crate) fn from_sink_match(
        sunk: &'a SinkMatch<'a>,
        original_matches: &'a [Match],
        // replacement: Option<(&'a [u8], &'a [Match])>,
    ) -> Sunk<'a> {
        // let (bytes, matches) =
        //     replacement.unwrap_or_else(|| (sunk.bytes(), original_matches));
        Sunk {
            bytes: sunk.bytes(),
            absolute_byte_offset: sunk.absolute_byte_offset(),
            line_number: sunk.line_number(),
            // context_kind: None,
            matches: original_matches,
            original_matches,
        }
    }

    #[inline]
    pub(crate) fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    #[inline]
    pub(crate) fn matches(&self) -> &'a [Match] {
        self.matches
    }

    #[inline]
    pub(crate) fn absolute_byte_offset(&self) -> u64 {
        self.absolute_byte_offset
    }

    #[inline]
    pub(crate) fn line_number(&self) -> Option<u64> {
        self.line_number
    }
}

/// 十进制数格式化器
#[derive(Debug)]
pub(crate) struct DecimalFormatter {
    /// 十进制数按位提取数字字符， 比如 12 -> ['0', .., '1', '2']
    buf: [u8; Self::MAX_U64_LEN],
    /// 指向最高有效位在数组的中索引, 比如 12 转换后 start = 18
    start: usize,
}

impl DecimalFormatter {
    /// Discovered via `u64::MAX.to_string().len()`.
    const MAX_U64_LEN: usize = 20;  //u64数字的位数最大为20

    pub(crate) fn new(mut n: u64) -> DecimalFormatter {
        let mut buf = [0; Self::MAX_U64_LEN];
        let mut i = buf.len();
        loop {
            i -= 1;

            let digit = u8::try_from(n % 10).unwrap();
            n /= 10;
            buf[i] = b'0' + digit;
            if n == 0 {
                break;
            }
        }
        DecimalFormatter { buf, start: i }
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.buf[self.start..]
    }
}

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

/// 迭代查找 bytes[range] 中所有匹配的字符串交给闭包中的 matched 处理
pub(crate) fn find_iter_at_in_context<M: Matcher, F: FnMut(Match) -> bool>(
    searcher: &Searcher,
    matcher: M,
    mut bytes: &[u8],   //缓冲
    range: std::ops::Range<usize>,  //匹配行在缓冲中的范围
    mut matched: F,
) -> io::Result<()> {
    let mut m = Match::new(0, range.end);
    trim_line_terminator(searcher, bytes, &mut m);
    bytes = &bytes[..m.end()];
    matcher
        .find_iter_at(bytes, range.start, |m| {
            if m.start() >= range.end {
                return false;
            }
            matched(m)
        })
        .map_err(io::Error::error_message)
}

/// 清除行中的行终止符，从前面的逻辑看如果行中有行终止符，只可能在最后面，所以这里只需要判断最后一位是否是行终止符
pub(crate) fn trim_line_terminator(
    searcher: &Searcher,
    buf: &[u8],
    line: &mut Match,
) {
    let line_terminator = searcher.line_terminator();
    if line_terminator.is_suffix(&buf[*line]) {
        let mut end = line.end() - 1;
        if line_terminator.is_crlf() && end > 0 && buf.get(end - 1) == Some(&b'\r') {
            end -= 1;
        }
        *line = line.with_end(end);
    }
}
