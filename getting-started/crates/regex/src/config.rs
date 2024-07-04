use regex_automata::meta::Regex;
use regex_syntax::{ast, hir};
use regex_syntax::hir::Hir;
use grep_matcher::{ByteSet, LineTerminator};
use crate::ast::AstAnalysis;
use crate::ban;
use crate::error::Error;
use crate::non_matching::non_matching_bytes;
use crate::strip::strip_from_match;

/// 正则表达式规则配置, 有些是 Ast 配置，有些是 Hir 配置，下面的配置项的说明是 GPT 给的，TODO 待验证是否正确
#[derive(Clone, Debug)]
pub(crate) struct Config {
    /// 如果设置为 true，则在正则表达式匹配时忽略大小写
    pub(crate) case_insensitive: bool,
    /// 如果设置为 true，可能会根据输入文本的具体情况智能地应用大小写敏感或不敏感的匹配 ？？？
    pub(crate) case_smart: bool,
    /// 如果设置为 true，多行模式将启用，这意味着 ^ 和 $ 分别匹配每一行的开始和结束，而不是整个输入的开始和结束
    pub(crate) multi_line: bool,
    /// 如果设置为 true，则点（.）字符将匹配包括换行符在内的任何字符。
    pub(crate) dot_matches_new_line: bool,
    /// 如果设置为 true，贪婪量词（如 *, +, ?, {...}）将变为非贪婪想理解清里面的原理，反之亦然 ？？？
    pub(crate) swap_greed: bool,
    /// 如果设置为 true，则在正则表达式中忽略空白字符，除非它们被转义或在字符类中
    pub(crate) ignore_whitespace: bool,
    /// 如果设置为 true，启用对 Unicode 属性的支持
    pub(crate) unicode: bool,
    /// 如果设置为 true，允许在正则表达式中使用八进制转义序列
    pub(crate) octal: bool,
    /// 正整数，限制正则表达式编译过程中可以使用的内存量
    pub(crate) size_limit: usize,
    /// 正整数，限制确定性有限自动机（DFA）状态机的大小
    pub(crate) dfa_size_limit: usize,
    /// 无符号32位整数，限制正则表达式中嵌套量词的最大深度
    pub(crate) nest_limit: u32,
    /// 枚举，定义了正则表达式中用于匹配行终止符的类型
    pub(crate) line_terminator: Option<LineTerminator>,
    /// 表示在正则表达式中禁止使用的字符
    pub(crate) ban: Option<u8>,
    /// 如果设置为 true，启用对回车换行（CRLF）的特定处理
    pub(crate) crlf: bool,
    /// 如果设置为 true，启用对单词边界的特殊处理 ？？？
    pub(crate) word: bool,
    /// 如果设置为 true，启用对固定字符串的特殊处理 ？？？
    pub(crate) fixed_strings: bool,
    /// 如果设置为 true，将整个输入视为一行，^ 和 $ 将匹配整个输入的开始和结束
    pub(crate) whole_line: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            case_insensitive: false,        //默认区分大小写
            case_smart: false,
            multi_line: false,              //默认不是多行模式
            dot_matches_new_line: false,
            swap_greed: false,
            ignore_whitespace: false,
            unicode: true,
            octal: false,
            size_limit: 100 * (1 << 20),
            dfa_size_limit: 1000 * (1 << 20),
            nest_limit: 250,
            line_terminator: None,
            ban: None,
            crlf: false,
            word: false,
            fixed_strings: false,
            whole_line: false,
        }
    }
}

impl Config {
    pub(crate) fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<ConfiguredHIR, Error> {
        ConfiguredHIR::new(self.clone(), patterns)
    }

    fn is_case_insensitive(&self, analysis: &AstAnalysis) -> bool {
        if self.case_insensitive {
            return true;
        }
        if !self.case_smart {
            return false;
        }
        analysis.any_literal() && !analysis.any_uppercase()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ConfiguredHIR {
    config: Config,
    /// high-level intermediate representation, 意为高级中间表示
    hir: Hir,
}

impl ConfiguredHIR {
    /// 这里可以看到 patterns 转成 Hir 的流程 (patterns -> Ast -> Hir)
    fn new<P: AsRef<str>>(config: Config, patterns: &[P]) -> Result<ConfiguredHIR, Error> {
        // 先忽略对固定字符串的处理
        // let hir = if config.is_fixed_strings(patterns) {    //判断patterns是否全是固定字符串（即不包含正则表达式字符不包含行终止符），如果不关心大小写所有pattern都不当作固定字符串处理
        // } else {
        // }
        // 非固定字符串处理方式
        // 1 先将多个模式字符串使用"|"拼接成一个模式字符串
        let mut alts = vec![];
        for p in patterns.iter() {
            alts.push(if config.fixed_strings {
                format!("(?:{})", regex_syntax::escape(p.as_ref())) //TODO
            } else {
                format!("(?:{})", p.as_ref())
            });
        }
        let pattern = alts.join("|");
        // 后面的流程看不懂，因为对正则表达式引擎的工作原理和实现不清楚，不过也不是短时间就能理清的，暂时不纠结了，后面有空再看 TODO 正则表达式引擎工作原理
        // 2 Ast
        let mut parser = ast::parse::ParserBuilder::new()
            .nest_limit(config.nest_limit)
            .octal(config.octal)
            .ignore_whitespace(config.ignore_whitespace)
            .build();
        let ast = parser.parse(&pattern)
            .map_err(Error::generic)?;
        // 3 Hir
        let analysis = AstAnalysis::from_ast(&ast);
        let mut hir = hir::translate::TranslatorBuilder::new()
            .utf8(false)
            .case_insensitive(config.is_case_insensitive(&analysis))
            .multi_line(config.multi_line)
            .dot_matches_new_line(config.dot_matches_new_line)
            .crlf(config.crlf)
            .swap_greed(config.swap_greed)
            .unicode(config.unicode)
            .build()
            .translate(&pattern, &ast)
            .map_err(Error::generic)?;
        if let Some(byte) = config.ban {
            ban::check(&hir, byte)?;
        }
        hir = match config.line_terminator {
            None => hir,
            Some(line_term) => strip_from_match(hir, line_term)?,
        };
        Ok(ConfiguredHIR { config, hir })
    }

    pub(crate) fn config(&self) -> &Config {
        &self.config
    }

    pub(crate) fn hir(&self) -> &Hir {
        &self.hir
    }

    /// Hir -> Regex
    pub(crate) fn to_regex(&self) -> Result<Regex, Error> {
        let meta = Regex::config()
            .utf8_empty(false)
            .nfa_size_limit(Some(self.config.size_limit))
            // We don't expose a knob for this because the one-pass DFA is
            // usually not a perf bottleneck for ripgrep. But we give it some
            // extra room than the default.
            .onepass_size_limit(Some(10 * (1 << 20)))
            // Same deal here. The default limit for full DFAs is VERY small,
            // but with ripgrep we can afford to spend a bit more time on
            // building them I think.
            .dfa_size_limit(Some(1 * (1 << 20)))
            .dfa_state_limit(Some(1_000))
            .hybrid_cache_capacity(self.config.dfa_size_limit);
        Regex::builder()
            .configure(meta)
            .build_from_hir(&self.hir)
            .map_err(Error::regex)
    }

    pub(crate) fn non_matching_bytes(&self) -> ByteSet {
        non_matching_bytes(&self.hir)
    }

    pub(crate) fn line_terminator(&self) -> Option<LineTerminator> {
        if self.hir.properties().look_set().contains_anchor_haystack() {
            None
        } else {
            self.config.line_terminator
        }
    }
}