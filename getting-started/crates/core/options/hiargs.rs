use std::collections::HashSet;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use termcolor::{ColorChoice, StandardStream, WriteColor};
use walkdir::WalkDir;
use grep::printer::StandardBuilder;
use grep::regex::RegexMatcherBuilder;
use grep::searcher::{Searcher, SearcherBuilder};
use crate::options::lowargs::{CaseMode, LowArgs, Mode, PatternSource, SearchMode};
use crate::search::{PatternMatcher, Printer, SearchWorker, SearchWorkerBuilder};

/// HiArgs 是实际应用到各个组件的参数，LowArgs 中的参数类型一般都比较简单，在使用前基本需要进一步处理
#[derive(Debug)]    //这一句用于自动派生 Debug 这一 trait 的方法，trait 的定位有点类似其他语言的接口
pub(crate) struct HiArgs {
    mode: Mode,
    /// 匹配使用的 Pattern
    patterns: Patterns,
    /// 匹配的目标路径（文件或目录）
    paths: Paths,
    /// 大小写是否敏感
    case: CaseMode,
    // 颜色高亮输出颜色选择
    // color: ColorChoice,
    /// 是否打印匹配项在匹配行中的列数
    column: bool,
    /// 是否以标题的方式打印匹配文件路径
    heading: bool,
    /// 是否打印匹配行在文件中的行号
    line_number: bool,
    /// 自定义的路径分隔符
    path_terminator: Option<u8>,
    /// 搜索使用线程数量
    threads: usize,
}

impl HiArgs {
    /// LowArgs 转 HiArgs
    pub(crate) fn from_low_args(mut low: LowArgs) -> anyhow::Result<HiArgs> {
        let mut state = State::new()?;  //主要是cwd
        // 匹配Pattern (Vec<PatternResource> -> Vec<String>)
        let patterns = Patterns::from_low_args(&mut state, &mut low)?;
        // 路径处理 (先从 positional 中找，没有就使用当期工作目录)
        let paths = Paths::from_low_args(&mut state, &patterns, &mut low)?;
        // 是否打印匹配项列号
        let column = low.column.unwrap_or(false);
        // 是否按标题形式打印所属文件路径
        let heading = low.heading.unwrap_or(false);
        // 是否打印匹配行行号
        let line_number = low.line_number.unwrap_or_else(|| {   //即便没设置，如果设置的是标准搜索模式且打印列号就也打印行号
            let Mode::Search(ref search_mode) = low.mode else { return false };
            match *search_mode {
                SearchMode::Standard => {
                    state.is_terminal_stdout
                        || column
                }
            }
        });
        // 搜索线程数，多线程搜索只是适用于多文件搜索（ripgrep 对于结果需要排序的情况也不使用多线程搜索）
        let threads = if paths.is_one_file {
            1
        } else if let Some(threads) = low.threads {
            1
            // threads  //TODO
        } else {
            // 没有设置使用多少线程数且是多文件搜索，就选择 min(CPU核心数,12)
            // std::thread::available_parallelism().map_or(1, |n| n.get()).min(12)
            1           //TODO
        };

        Ok(HiArgs {
            mode: low.mode,
            patterns,
            paths,
            case: low.case,
            // color,
            column,
            heading,
            line_number,
            path_terminator: low.path_separator,
            threads,
        })
    }

    pub(crate) fn mode(&self) -> Mode {
        self.mode
    }

    pub(crate) fn matches_possible(&self) -> bool {
        if self.patterns.patterns.is_empty() {
            return false;
        }
        true
    }

    pub(crate) fn threads(&self) -> usize {
        self.threads
    }

    pub(crate) fn search_worker<W: WriteColor>(
        &self,
        matcher: PatternMatcher,
        searcher: grep::searcher::Searcher,
        printer: Printer<W>,
    ) -> anyhow::Result<SearchWorker<W>> {
        let builder = SearchWorkerBuilder::new();
        Ok(builder.build(searcher, matcher, printer))
    }

    pub(crate) fn matcher(&self) -> anyhow::Result<PatternMatcher> {
        let mut builder = RegexMatcherBuilder::new();
        match self.case {
            CaseMode::Sensitive => builder.case_insensitive(true),
            CaseMode::Insensitive => builder.case_insensitive(false),
            CaseMode::Smart => builder.case_smart(true),
        };
        let m = match builder.build_many(&self.patterns.patterns) { // String 实现了 AsRef<str>
            Ok(m) => m,
            Err(err) => {
                anyhow::bail!("error build matcher: {}", err.to_string());
            }
        };
        Ok(PatternMatcher::RustRegex(m))
    }

    pub(crate) fn searcher(&self) -> anyhow::Result<Searcher> {
        let mut builder = SearcherBuilder::new();
        builder.line_number(self.line_number);
        Ok(builder.build())
    }

    pub(crate) fn printer<W: WriteColor>(
        &self,
        _: SearchMode,
        wtr: W,
    ) -> Printer<W> {
        let standard = StandardBuilder::new()
            .column(self.column)
            .heading(self.heading)
            .path_terminator(self.path_terminator.clone())
            .max_columns(Some(4096))
            .trim_ascii(true)
            .build(wtr);
        Printer::Standard(standard)
    }

    pub(crate) fn paths(&mut self) -> Vec<PathBuf> {
        let mut file_paths = Vec::new();
        if self.paths.is_one_file { //如果只是一个文件
            file_paths.push(self.paths.paths[0].clone());
            return file_paths;
        }
        // 如果是目录，需要递归遍历目录，获取所有文件的路径
        for path in self.paths.paths.drain(..) {
            if path.is_file() {
                file_paths.push(path);
                continue;
            }
            // 使用 WalkDir 遍历目录
            for entry in WalkDir::new(path) {
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("Error accessing entry: {}", e);
                        continue;
                    }
                };
                // 确保是文件
                if entry.file_type().is_file() {
                    file_paths.push(entry.path().to_path_buf());
                }
            }
        }
        file_paths
    }

    pub(crate) fn stdout(&self) -> StandardStream {
        StandardStream::stdout(ColorChoice::Auto)
    }
}

#[derive(Debug)]
struct State {
    /// tty 已连接到标准输出
    is_terminal_stdout: bool,
    /// 标准输入是否已经被消费
    // stdin_consumed: bool,
    /// 当前工作目录路径
    cwd: PathBuf,
}

impl State {
    fn new() -> anyhow::Result<State> {
        use std::io::IsTerminal;
        Ok(State {
            is_terminal_stdout: std::io::stdout().is_terminal(),
            // stdin_consumed: false,
            cwd: current_dir()?,    //env.rs
        })
    }
}

#[derive(Debug)]
struct Patterns {
    // 匹配用的 pattern
    patterns: Vec<String>,
}

impl Patterns {
    // LowArgs patterns -> HiArgs patterns
    // 优先使用 -e/--regexp 指定的正则表达式，没有就使用 positional args 中的第一个参数
    fn from_low_args(state: &mut State, low: &mut LowArgs) -> anyhow::Result<Patterns> {
        //除了 Search 模式其他不需要 Pattern
        if !matches!(low.mode, Mode::Search(_)) {
            return Ok(Patterns { patterns: vec![] });
        }
        if low.patterns.is_empty() {    //即没有通过 -e/--regexp 指定正则表达式, 选择 positional 中的第一个参数
            anyhow::ensure!(
                !low.positional.is_empty(),
                "ripgrep requires at least one pattern to execute a search"
            );
            let os_pattern = low.positional.remove(0);   //从 Vec 中删除索引为0的元素并返回
            let Ok(pattern) = os_pattern.into_string() else {
                anyhow::bail!("pattern given is not valid UTF-8");
            };
            return Ok(Patterns { patterns: vec![pattern] });
        }
        //使用 -e/--regexp 指定的正则表达式
        //去重并转 PatternSource -> String
        let mut seen = HashSet::new();
        let mut patterns = Vec::with_capacity(low.patterns.len());
        // for source in low.patterns {  //这种方式会导致low.pattens所有权转移
        for source in low.patterns.drain(..) {
            match source {
                PatternSource::Regexp(pat) => {
                    if !seen.contains(&pat) {
                        seen.insert(pat.clone());
                        patterns.push(pat);
                    }
                }
            }
        }
        Ok(Patterns { patterns })
    }
}

#[derive(Debug, Clone)]
struct Paths {
    /// 待搜索的路径
    paths: Vec<PathBuf>,
    /// 是否是隐式的搜索路径（即通过参数传递的路径），没有就使用当前路径，ripgrep 还支持从标准输入读取路径（这种方式暂略）
    has_implicit_path: bool,
    /// 路径是否是一个文件
    is_one_file: bool,
}

impl Paths {
    /// 从命令行参数获取路径参数，并获取最终的路径,看来路径也可以带多个
    fn from_low_args(state: &mut State, _: &Patterns, low: &mut LowArgs) -> anyhow::Result<Paths> {
        let mut paths = Vec::with_capacity(low.positional.len());   //positional 中剩下的参数都默认是路径参数
        for os_arg in low.positional.drain(..) {
            let path = PathBuf::from(os_arg);
            paths.push(path);
        }
        log::debug!("number of paths given to search: {}", paths.len());
        if !paths.is_empty() {
            let is_one_file = paths.len() == 1 && !paths[0].is_dir(); //
            log::debug!("is_one_file? {is_one_file:?}");
            return Ok(Paths { paths, has_implicit_path: false, is_one_file });
        }
        // paths 为空就使用当前工作目录
        log::debug!("heuristic chose to search ./");
        Ok(Paths { paths: vec![PathBuf::from("./")], has_implicit_path: true, is_one_file: false })
    }
}