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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum LineTerminatorImp {
    /// Any single byte representing a line terminator.
    Byte(u8),
    /// 使用 `\r\n` 作为行终止符，同时仍然会将单独的 `\n` 视为行终止符
    CRLF,
}

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
}

pub trait Matcher {

    type Error: std::fmt::Display;

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

    #[inline]
    fn shortest_match_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<usize>, Self::Error> {
        Ok(self.find_at(haystack, at)?.map(|m| m.end))
    }

    fn find_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<Match>, Self::Error>;
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