/// 搜索过程中的一些统计数据
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    // searches: u64,
    // searches_with_match: u64,
    // bytes_searched: u64,
    // bytes_printed: u64,
    /// 搜索过程中匹配的行数
    matched_lines: u64,
    /// 搜索过程中匹配行的总字节数
    matches: u64,
}

impl Stats {
    pub fn new() -> Stats {
        Stats::default()
    }

    pub fn add_matched_lines(&mut self, n: u64) {
        self.matched_lines += n;
    }

    pub fn add_matches(&mut self, n: u64) {
        self.matches += n;
    }
}