use std::io;
use std::path::Path;
use {grep::matcher::Matcher, termcolor::WriteColor};
use grep::searcher::Searcher;

#[derive(Clone, Debug, Default)]
struct Config {}

#[derive(Clone, Debug)]
pub(crate) struct SearchWorkerBuilder {
    config: Config,
}

impl Default for SearchWorkerBuilder {
    fn default() -> SearchWorkerBuilder {
        SearchWorkerBuilder::new()
    }
}

impl SearchWorkerBuilder {
    pub(crate) fn new() -> SearchWorkerBuilder {
        SearchWorkerBuilder {
            config: Config::default(),
        }
    }

    pub(crate) fn build<W: WriteColor>(
        &self,
        searcher: Searcher,
        matcher: PatternMatcher,
        printer: Printer<W>,
    ) -> SearchWorker<W> {
        let config = self.config.clone();
        SearchWorker {
            config,
            searcher,
            matcher,
            printer,
        }
    }
}

/// 核心类
/// 执行搜索的 Worker，内部分别利用 LineBuffer、RegexMatcher、StandardImpl 实现文本读取、匹配、以及输出
/// 搜索流程内部还涉及了好多中间类型，梳理清这些类型的关系对梳理整个搜索流程比较重要
/// SearcherWorker
///     -> searcher: Searcher
///
///     -> matcher: PatternMatcher
///         -> RegexMatcher
///     -> printer: Printer
///         -> Standard
#[derive(Clone, Debug)]
pub(crate) struct SearchWorker<W> {
    config: Config,
    /// 内部封装了 LineBuffer，但是功能并不局限于文件读取到缓冲，从 ripgrep 源码看 Searcher 才是真正实现了整个搜索流程的类型，
    /// 正则匹配和结果输出则是通过方法传参由 searcher 调用 matcher、printer 实现
    searcher: grep::searcher::Searcher,
    /// 实现消费缓冲数据进行正则匹配， 是对 RegexMatcher(实现 Matcher 特征)的代理封装
    matcher: PatternMatcher,
    /// 实现将匹配结果输出， 是对 Standard 的代理封装
    printer: Printer<W>,
}

impl<W: WriteColor> SearchWorker<W> {
    /// 核心方法
    pub(crate) fn search(
        &mut self,
        path: &Path,
    ) -> io::Result<SearchResult> {
        log::debug!("search path: {}", path.display());

        // ripgrep 这里支持多种处理，比如从标准输入搜索、执行搜索前预处理、从压缩路径搜索（会先解压）
        // 这里只展示从普通的文件路径搜索
        self.search_path(path)
    }

    /// 从文件路径指定的文件搜索
    fn search_path(&mut self, path: &Path) -> io::Result<SearchResult> {
        // 获取 searcher printer 可变引用
        let (searcher, printer) = (&mut self.searcher, &mut self.printer);
        match self.matcher {
            // ref m: 指匹配并获取matcher的引用，来避免所有权转移
            PatternMatcher::RustRegex(ref m) => {
                search_path(m, searcher, printer, path)
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SearchResult {
    //是否有匹配的行
    has_match: bool,
}

impl SearchResult {
    pub(crate) fn has_match(&self) -> bool {
        self.has_match
    }
}

/// 支持的正则引擎匹配器，这里只展示 Rust Regex
/// Pattern -> RegexMatcher
#[derive(Clone, Debug)]
pub(crate) enum PatternMatcher {
    RustRegex(grep::regex::RegexMatcher),
    // #[cfg(feature = "pcre2")]
    // PCRE2(grep::pcre2::RegexMatcher),
}

/// 匹配结果的输出类型
/// ripgrep 支持三种实现 Standard Summary JSON, 这里只分析 Standard
#[derive(Clone, Debug)]
pub(crate) enum Printer<W> {
    /// Use the standard printer, which supports the classic grep-like format.
    Standard(grep::printer::Standard<W>),
    // Use the summary printer, which supports aggregate displays of search results.
    // Summary(grep::printer::Summary<W>),
    // A JSON printer, which emits results in the JSON Lines format.
    // JSON(grep::printer::JSON<W>),
}

fn search_path<M: Matcher, W: WriteColor>(
    matcher: M,
    searcher: &mut grep::searcher::Searcher,
    printer: &mut Printer<W>,
    path: &Path,
) -> io::Result<SearchResult> {
    match *printer {
        Printer::Standard(ref mut standard) => {
            let mut sink = standard.sink_with_path(&matcher, path);
            searcher.search_path(&matcher, path, &mut sink)?;     //TODO 为何这里 &sink 不可变引用会报编译错误： the trait `grep::grep_searcher::Sink` is not implemented for `&printer::standard::StandardSink<'_, '_, &M, W>`
            //官方推荐要么传值、要么使用可变引用；
            Ok(SearchResult {
                has_match: sink.has_match(),    //是否有搜索到匹配行
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use termcolor::ColorChoice;
    use grep::printer::StandardBuilder;
    use grep::regex::RegexMatcherBuilder;
    use grep::searcher::SearcherBuilder;
    use crate::search::{PatternMatcher, Printer, SearchWorkerBuilder};

    #[test]
    fn do_search() {
        // 1 构建 SearchWorker 及内部组件
        //  searcher
        let searcher = SearcherBuilder::new()
            .build();
        //  matcher
        let matcher = PatternMatcher::RustRegex(RegexMatcherBuilder::new()
            .build("grep").unwrap());
        //  printer
        let out = termcolor::StandardStream::stdout(ColorChoice::Auto);
        let standard = StandardBuilder::new()
            .max_columns(Some(4096))
            .trim_ascii(true)
            .build(out);
        let printer = Printer::Standard(standard);
        //  search_worker
        let builder = SearchWorkerBuilder::new();
        let mut search_worker = builder.build(searcher, matcher, printer);
        // 2 执行搜索、输出等流程
        //  这里的例子是搜索根目录下 Cargo.toml 中包含 grep 的行
        let path = Path::new("./Cargo.toml");
        search_worker.search(path).unwrap();
    }
}