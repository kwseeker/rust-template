use regex_syntax::hir::{
    self, ClassBytesRange, ClassUnicodeRange, Hir, HirKind,
};
use crate::error::{Error, ErrorKind};

pub(crate) fn check(expr: &Hir, byte: u8) -> Result<(), Error> {
    assert!(byte.is_ascii(), "ban byte must be ASCII");
    let ch = char::from(byte);
    let invalid = || Err(Error::new(ErrorKind::Banned(byte)));
    match expr.kind() {
        HirKind::Empty => {}
        HirKind::Literal(hir::Literal(ref lit)) => {
            if lit.iter().find(|&&b| b == byte).is_some() {
                return invalid();
            }
        }
        HirKind::Class(hir::Class::Unicode(ref cls)) => {
            if cls.ranges().iter().map(|r| r.len()).sum::<usize>() == 1 {
                let contains =
                    |r: &&ClassUnicodeRange| r.start() <= ch && ch <= r.end();
                if cls.ranges().iter().find(contains).is_some() {
                    return invalid();
                }
            }
        }
        HirKind::Class(hir::Class::Bytes(ref cls)) => {
            if cls.ranges().iter().map(|r| r.len()).sum::<usize>() == 1 {
                let contains = |r: &&ClassBytesRange| {
                    r.start() <= byte && byte <= r.end()
                };
                if cls.ranges().iter().find(contains).is_some() {
                    return invalid();
                }
            }
        }
        HirKind::Look(_) => {}
        HirKind::Repetition(ref x) => check(&x.sub, byte)?,
        HirKind::Capture(ref x) => check(&x.sub, byte)?,
        HirKind::Concat(ref xs) => {
            for x in xs.iter() {
                check(x, byte)?;
            }
        }
        HirKind::Alternation(ref xs) => {
            for x in xs.iter() {
                check(x, byte)?;
            }
        }
    };
    Ok(())
}