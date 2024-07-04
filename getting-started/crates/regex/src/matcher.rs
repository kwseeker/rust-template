use regex_automata::Input;
use regex_automata::meta::Regex;
use grep_matcher::{ByteSet, LineTerminator, Match, Matcher, NoError};
use crate::config::Config;
use crate::error::Error;
use crate::literals::InnerLiterals;

/// ripgrep 搜索流程的3个重要的类型之一 RegexMatcher (ripgrep支持两种正则引擎：Rust Regex、PCRE2，这里只展示 Rust Regex)
/// RegexMatcher 用于执行正则表达式匹配，匹配符合 PATTERN 参数的行
/// 这里使用的正则表达式包是 regex-automata （一个使用确定性有限自动机(DFA)的低级正则表达式库） TODO 有限自动机 ？
/// https://docs.rs/regex-automata
/// https://github.com/BurntSushi/regex-automata
/// 这个包现在已经被纳入官方的 regex 包，https://github.com/rust-lang/regex, 包括 regex-syntax
/// 正则匹配工作大概流程：
/// 1）先将正则表达式字符串经过 regex_syntax 语法解析器转成 Hir
/// 2) 通过 Hir 创建正则匹配器 Regex
/// 3) 匹配器匹配字符串获取匹配结果
/// 正则引擎详细工作流程还是较复杂的，可以参考 《精通正则表达式》略窥一二，应该可以帮助理解正则表达式中的配置项的使用(浏览了下发现没帮助)

#[derive(Clone, Debug)]
pub struct RegexMatcherBuilder {
    config: Config,
}

impl Default for RegexMatcherBuilder {
    fn default() -> RegexMatcherBuilder {
        RegexMatcherBuilder::new()
    }
}

impl RegexMatcherBuilder {
    pub fn new() -> RegexMatcherBuilder {
        RegexMatcherBuilder { config: Config::default() }
    }

    pub fn build(&self, pattern: &str) -> Result<RegexMatcher, Error> {
        self.build_many(&[pattern])
    }

    /// 关键方法
    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<RegexMatcher, Error> {
        let mut chir = self.config.build_many(patterns)?;

        let regex = chir.to_regex()?;
        log::debug!("final regex: {:?}", chir.hir().to_string());

        let non_matching_bytes = chir.non_matching_bytes();
        let fast_line_regex = InnerLiterals::new(&chir, &regex).one_regex()?;

        let mut config = self.config.clone();
        config.line_terminator = chir.line_terminator();
        Ok(RegexMatcher { config, regex, fast_line_regex, non_matching_bytes })
    }

    // 后面都是些配置定制方法 --------------------------------------------------------------
    /// 设置是否启用多行模式
    pub fn multi_line(&mut self, yes: bool) -> &mut RegexMatcherBuilder {
        self.config.multi_line = yes;
        self
    }

    /// 通过设置 Config 配置所有匹配都发生在单词边界上
    pub fn word(&mut self, yes: bool) -> &mut RegexMatcherBuilder {
        self.config.word = yes;
        self
    }

    pub fn unicode(&mut self, yes: bool) -> &mut RegexMatcherBuilder {
        self.config.unicode = yes;
        self
    }

    pub fn octal(&mut self, yes: bool) -> &mut RegexMatcherBuilder {
        self.config.octal = yes;
        self
    }

    pub fn fixed_strings(&mut self, yes: bool) -> &mut RegexMatcherBuilder {
        self.config.fixed_strings = yes;
        self
    }

    pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexMatcherBuilder {
        self.config.case_insensitive = yes;
        self
    }

    pub fn dot_matches_new_line(
        &mut self,
        yes: bool,
    ) -> &mut RegexMatcherBuilder {
        self.config.dot_matches_new_line = yes;
        self
    }

    pub fn line_terminator(
        &mut self,
        line_term: Option<u8>,
    ) -> &mut RegexMatcherBuilder {
        self.config.line_terminator = line_term.map(LineTerminator::byte);
        self
    }

    pub fn ban_byte(&mut self, byte: Option<u8>) -> &mut RegexMatcherBuilder {
        self.config.ban = byte;
        self
    }
}

#[derive(Clone, Debug)]
pub struct RegexMatcher {
    config: Config,
    /// 从 PATTERN 参数编译的正则表达式
    regex: Regex,
    fast_line_regex: Option<Regex>,
    non_matching_bytes: ByteSet,
}

impl Matcher for RegexMatcher {

    type Error = NoError;

    #[inline]
    fn find_at(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<Match>, NoError> {
        let input = Input::new(haystack).span(at..haystack.len());
        Ok(self.regex.find(input).map(|m| Match::new(m.start(), m.end())))
    }
}

#[cfg(test)]
mod tests {
    use grep_matcher::Matcher;
    use crate::matcher::RegexMatcherBuilder;

    /// 参考 matcher_rust() 方法的流程
    #[test]
    fn ripgrep_default_match() {
        let mut builder = RegexMatcherBuilder::new();
        builder.multi_line(true)
            .unicode(true)
            .octal(false)
            .fixed_strings(false)
            .case_insensitive(true)
            .dot_matches_new_line(false)
            .line_terminator(Some(b'\n'))
            .ban_byte(Some(b'\x00'));

        let patterns = vec![String::from("complex")];
        let matcher = builder.build_many(&patterns).unwrap();
        let result = matcher.is_match("regex engine is complex".as_bytes()).unwrap();
        assert_eq!(true, result);

        let patterns = vec![String::from("rgx")];
        let matcher = builder.build_many(&patterns).unwrap();
        let result = matcher.is_match("regex engine is complex".as_bytes()).unwrap();
        assert_eq!(false, result);

        let patterns = vec![String::from("^[A-Za-z\\s]+$")];    //匹配全部只包含英文字母和空白字符的字符串
        let matcher = builder.build_many(&patterns).unwrap();
        let result = matcher.is_match("regex engine is complex".as_bytes()).unwrap();
        let result2 = matcher.is_match("2 regex engine is complex".as_bytes()).unwrap();
        assert_eq!(true, result);
        assert_eq!(false, result2);
    }
}