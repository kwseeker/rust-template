use std::io;

/// 指向可寻址内存的连续块的可能为空的范围。
/// 其实就是用于表示匹配字符串范围的，这里的匹配字符串可能是行、可能是行里匹配的字符串
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Match {
    start: usize,
    end: usize,
}

impl Match {
    #[inline]
    pub fn new(start: usize, end: usize) -> Match {
        assert!(start <= end);
        Match { start, end }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    #[inline]
    pub fn with_start(&self, start: usize) -> Match {
        Match { start, ..*self }    //结构体更新，这里只更新了 start, 其他值用的原值
    }

    #[inline]
    pub fn with_end(&self, end: usize) -> Match {
        assert!(self.start <= end, "{} is not <= {}", self.start, end);
        Match { end, ..*self }
    }

    #[inline]
    pub fn zero(offset: usize) -> Match {
        Match {start: offset, end: offset}
    }

    #[inline]
    pub fn offset(&self, amount: usize) -> Match {
        Match {
            start: self.start.checked_add(amount).unwrap(), //checked_add 会检查是否溢出，溢出时返回 None
            end: self.end.checked_add(amount).unwrap(),
        }
    }
}

/// 为了使用 container[index] 这个容器的语法糖， container 需要实现 std::ops::Index 特征
impl std::ops::Index<Match> for [u8] {
    type Output = [u8];

    #[inline]
    fn index(&self, index: Match) -> &[u8] {
        &self[index.start..index.end]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LineTerminator(LineTerminatorImp);

impl Default for LineTerminator {
    #[inline]
    fn default() -> LineTerminator {
        LineTerminator::byte(b'\n')
    }
}

impl LineTerminator {
    #[inline]
    pub fn byte(byte: u8) -> LineTerminator {
        LineTerminator(LineTerminatorImp::Byte(byte))
    }

    #[inline]
    pub fn crlf() -> LineTerminator {
        LineTerminator(LineTerminatorImp::CRLF)
    }

    #[inline]
    pub fn is_crlf(&self) -> bool {
        self.0 == LineTerminatorImp::CRLF
    }

    #[inline]
    pub fn as_byte(&self) -> u8 {
        match self.0 {
            LineTerminatorImp::Byte(byte) => byte,
            LineTerminatorImp::CRLF => b'\n',
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        match self.0 {
            LineTerminatorImp::Byte(ref byte) => std::slice::from_ref(byte),
            LineTerminatorImp::CRLF => &[b'\r', b'\n'],
        }
    }

    /// 判断当前 LineTerminator 是否是 slice 中的后缀
    #[inline]
    pub fn is_suffix(&self, slice: &[u8]) -> bool {
        slice.last().map_or(false, |&b| b == self.as_byte())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum LineTerminatorImp {
    /// Any single byte representing a line terminator.
    Byte(u8),
    /// 使用 `\r\n` 作为行终止符，同时仍然会将单独的 `\n` 视为行终止符
    CRLF,
}

///
#[derive(Clone, Debug)]
pub struct ByteSet(BitSet);

#[derive(Clone, Copy)]
struct BitSet([u64; 4]);

impl std::fmt::Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmtd = f.debug_set();
        for b in 0..=255 {
            if ByteSet(*self).contains(b) {
                fmtd.entry(&b);
            }
        }
        fmtd.finish()
    }
}

impl ByteSet {
    /// Create an empty set of bytes.
    #[inline]
    pub fn empty() -> ByteSet {
        ByteSet(BitSet([0; 4]))
    }

    /// Create a full set of bytes such that every possible byte is in the set
    /// returned.
    #[inline]
    pub fn full() -> ByteSet {
        ByteSet(BitSet([u64::MAX; 4]))
    }

    /// Add a byte to this set.
    ///
    /// If the given byte already belongs to this set, then this is a no-op.
    #[inline]
    pub fn add(&mut self, byte: u8) {
        let bucket = byte / 64;
        let bit = byte % 64;
        (self.0).0[usize::from(bucket)] |= 1 << bit;
    }

    /// Add an inclusive range of bytes.
    #[inline]
    pub fn add_all(&mut self, start: u8, end: u8) {
        for b in start..=end {
            self.add(b);
        }
    }

    /// Remove a byte from this set.
    ///
    /// If the given byte is not in this set, then this is a no-op.
    #[inline]
    pub fn remove(&mut self, byte: u8) {
        let bucket = byte / 64;
        let bit = byte % 64;
        (self.0).0[usize::from(bucket)] &= !(1 << bit);
    }

    /// Remove an inclusive range of bytes.
    #[inline]
    pub fn remove_all(&mut self, start: u8, end: u8) {
        for b in start..=end {
            self.remove(b);
        }
    }

    /// Return true if and only if the given byte is in this set.
    #[inline]
    pub fn contains(&self, byte: u8) -> bool {
        let bucket = byte / 64;
        let bit = byte % 64;
        (self.0).0[usize::from(bucket)] & (1 << bit) > 0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LineMatchKind {
    ///
    Confirmed(usize),
    ///
    Candidate(usize),
}

pub trait Matcher {
    type Error: std::fmt::Display;

    /// 查找字节数组 haystack 中是否有匹配的字符串，只要匹配到一项就立即返回
    #[inline]
    fn is_match(&self, haystack: &[u8]) -> Result<bool, Self::Error> {
        self.is_match_at(haystack, 0)
    }

    #[inline]
    fn is_match_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, Self::Error> {
        Ok(self.shortest_match_at(haystack, at)?.is_some())
    }

    /// 查找字节数组 haystack 中第一个匹配的字符串，返回第一个匹配的字符串在 haystack 中的范围的 end （封装到 Confirmed）
    fn find_candidate_line(&self, haystack: &[u8]) -> Result<Option<LineMatchKind>, Self::Error> {
        Ok(self.shortest_match(haystack)?.map(LineMatchKind::Confirmed))
    }

    #[inline]
    fn shortest_match(
        &self,
        haystack: &[u8],
    ) -> Result<Option<usize>, Self::Error> {
        self.shortest_match_at(haystack, 0)
    }

    /// 查找字节数组 haystack[at..] 中第一个匹配的字符串，返回第一个匹配的字符串在 haystack 中的范围的 end
    #[inline]
    fn shortest_match_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<usize>, Self::Error> {
        Ok(self.find_at(haystack, at)?.map(|m| m.end))
    }

    /// 核心匹配方法，
    /// 查找字节数组 haystack[at..] 中第一个匹配的字符串，返回第一个匹配的字符串在 haystack 中的范围
    /// 返回值：如果没有找到匹配项，返回Ok(None), 如果发生异常返回 Err(std::fmt::Display)
    fn find_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<Match>, Self::Error>;

    /// 迭代查询 bytes[at..] 中匹配的字符串交给 matched 闭包处理
    fn find_iter_at<F>(&self, bytes: &[u8], at: usize, mut matched: F) -> Result<(), Self::Error>
    where
        F: FnMut(Match) -> bool,
    {
        let mut last_end = at;
        loop {
            if last_end > bytes.len() {
                return Ok(())
            }
            let m = match self.find_at(bytes, last_end)? {
                None => return Ok(()),
                Some(m) => m,
            };
            last_end = m.end;
            match matched(m) {
                true => continue,
                false => return Ok(())
            }
        }
    }
}

/// 为所有实现了 Matcher 的类型重写 Matcher 下面的方法
impl<'a, M: Matcher> Matcher for &'a M {
    type Error = M::Error;

    #[inline]
    fn is_match(&self, haystack: &[u8]) -> Result<bool, Self::Error> {
        (*self).is_match(haystack)
    }

    #[inline]
    fn is_match_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, Self::Error> {
        (*self).is_match_at(haystack, at)
    }

    #[inline]
    fn shortest_match_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<usize>, Self::Error> {
        Ok((*self).find_at(haystack, at)?.map(|m| m.end))
    }

    fn find_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<Match>, Self::Error> {
        (*self).find_at(haystack, at)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct NoError(());

impl std::error::Error for NoError {
    fn description(&self) -> &str {
        "no error"
    }
}

impl std::fmt::Display for NoError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("BUG for NoError: an impossible error occurred")
    }
}

impl From<NoError> for std::io::Error {
    fn from(_: NoError) -> std::io::Error {
        panic!("BUG for NoError: an impossible error occurred")
    }
}