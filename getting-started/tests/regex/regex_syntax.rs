/// regex_syntax 提供了一个健壮的正则表达式解析器

use regex_syntax::{hir::Hir, parse};

#[test]
fn regex_syntax_usage() {
    let hir = parse("a|b")?;
    assert_eq!(hir, Hir::alternation(vec![
        Hir::literal("a".as_bytes()),
        Hir::literal("b".as_bytes()),
    ]));
}