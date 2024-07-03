use {
    grep_matcher::LineTerminator,
    regex_syntax::hir::{self, Hir, HirKind},
};

use crate::error::{Error, ErrorKind};

/// 返回保证永远不会匹配给定行终止符的 HIR
pub(crate) fn strip_from_match(
    expr: Hir,
    line_term: LineTerminator,
) -> Result<Hir, Error> {
    if line_term.is_crlf() {
        let expr1 = strip_from_match_ascii(expr, b'\r')?;
        strip_from_match_ascii(expr1, b'\n')
    } else {
        strip_from_match_ascii(expr, line_term.as_byte())
    }
}

fn strip_from_match_ascii(expr: Hir, byte: u8) -> Result<Hir, Error> {
    if !byte.is_ascii() {
        return Err(Error::new(ErrorKind::InvalidLineTerminator(byte)));
    }
    let ch = char::from(byte);
    let invalid = || Err(Error::new(ErrorKind::NotAllowed(ch.to_string())));
    Ok(match expr.into_kind() {
        HirKind::Empty => Hir::empty(),
        HirKind::Literal(hir::Literal(lit)) => {
            if lit.iter().find(|&&b| b == byte).is_some() {
                return invalid();
            }
            Hir::literal(lit)
        }
        HirKind::Class(hir::Class::Unicode(mut cls)) => {
            if cls.ranges().is_empty() {
                return Ok(Hir::class(hir::Class::Unicode(cls)));
            }
            let remove = hir::ClassUnicode::new(Some(
                hir::ClassUnicodeRange::new(ch, ch),
            ));
            cls.difference(&remove);
            if cls.ranges().is_empty() {
                return invalid();
            }
            Hir::class(hir::Class::Unicode(cls))
        }
        HirKind::Class(hir::Class::Bytes(mut cls)) => {
            if cls.ranges().is_empty() {
                return Ok(Hir::class(hir::Class::Bytes(cls)));
            }
            let remove = hir::ClassBytes::new(Some(
                hir::ClassBytesRange::new(byte, byte),
            ));
            cls.difference(&remove);
            if cls.ranges().is_empty() {
                return invalid();
            }
            Hir::class(hir::Class::Bytes(cls))
        }
        HirKind::Look(x) => Hir::look(x),
        HirKind::Repetition(mut x) => {
            x.sub = Box::new(strip_from_match_ascii(*x.sub, byte)?);
            Hir::repetition(x)
        }
        HirKind::Capture(mut x) => {
            x.sub = Box::new(strip_from_match_ascii(*x.sub, byte)?);
            Hir::capture(x)
        }
        HirKind::Concat(xs) => {
            let xs = xs
                .into_iter()
                .map(|e| strip_from_match_ascii(e, byte))
                .collect::<Result<Vec<Hir>, Error>>()?;
            Hir::concat(xs)
        }
        HirKind::Alternation(xs) => {
            let xs = xs
                .into_iter()
                .map(|e| strip_from_match_ascii(e, byte))
                .collect::<Result<Vec<Hir>, Error>>()?;
            Hir::alternation(xs)
        }
    })
}