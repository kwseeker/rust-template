use std::io;
use std::path::Path;

/// 执行搜索的Worker，核心结构体
#[derive(Clone, Debug)]
pub(crate) struct SearchWorker {
    /// 正则表达式匹配器
    /// ripgrep 支持 RustRegex 和 PCRE2 两种正则表达式引擎，这里暂时只展示 RustRegex
    matcher: grep::regex::RegexMatcher,
    /// 文本数据源
    searcher: grep::searcher::Searcher,
    /// 匹配输出目的地
    /// ripgrep 支持 Standard | Summary | JSON 三种格式，这里暂时只展示 Standard 格式
    printer: grep::printer::Standard,
}

impl SearchWorker {
    //
    pub(crate) fn search() -> io::Result<SearchResult> {

    }

    //
    pub(crate) fn search_path(&mut self, path: &Path) -> io::Result<SearchResult> {
        //searcher借助matcher搜索匹配的文本，并输出到printer
        self.searcher.search_path(&self.matcher, path, );
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SearchWorkerBuilder {
    config: Config,
}

impl SearchWorkerBuilder {
    pub(crate) fn new() -> SearchWorkerBuilder {

    }
}