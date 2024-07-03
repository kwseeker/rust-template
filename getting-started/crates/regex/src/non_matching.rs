use {
    grep_matcher::ByteSet,
    regex_syntax::{
        hir::{self, Hir, HirKind, Look},
        utf8::Utf8Sequences,
    },
};

/// Return a confirmed set of non-matching bytes from the given expression.
pub(crate) fn non_matching_bytes(expr: &Hir) -> ByteSet {
    let mut set = ByteSet::full();
    remove_matching_bytes(expr, &mut set);
    set
}

/// Remove any bytes from the given set that can occur in a matched produced by
/// the given expression.
fn remove_matching_bytes(expr: &Hir, set: &mut ByteSet) {
    match *expr.kind() {
        HirKind::Empty
        | HirKind::Look(Look::WordAscii | Look::WordAsciiNegate)
        | HirKind::Look(Look::WordUnicode | Look::WordUnicodeNegate)
        | HirKind::Look(Look::WordStartAscii | Look::WordStartUnicode)
        | HirKind::Look(Look::WordEndAscii | Look::WordEndUnicode)
        | HirKind::Look(
            Look::WordStartHalfAscii | Look::WordStartHalfUnicode,
        )
        | HirKind::Look(Look::WordEndHalfAscii | Look::WordEndHalfUnicode) => {
        }
        HirKind::Look(Look::Start | Look::End) => {
            // FIXME: This is wrong, but not doing this leads to incorrect
            // results because of how anchored searches are implemented in
            // the 'grep-searcher' crate.
            set.remove(b'\n');
        }
        HirKind::Look(Look::StartLF | Look::EndLF) => {
            set.remove(b'\n');
        }
        HirKind::Look(Look::StartCRLF | Look::EndCRLF) => {
            set.remove(b'\r');
            set.remove(b'\n');
        }
        HirKind::Literal(hir::Literal(ref lit)) => {
            for &b in lit.iter() {
                set.remove(b);
            }
        }
        HirKind::Class(hir::Class::Unicode(ref cls)) => {
            for range in cls.iter() {
                // This is presumably faster than encoding every codepoint
                // to UTF-8 and then removing those bytes from the set.
                for seq in Utf8Sequences::new(range.start(), range.end()) {
                    for byte_range in seq.as_slice() {
                        set.remove_all(byte_range.start, byte_range.end);
                    }
                }
            }
        }
        HirKind::Class(hir::Class::Bytes(ref cls)) => {
            for range in cls.iter() {
                set.remove_all(range.start(), range.end());
            }
        }
        HirKind::Repetition(ref x) => {
            remove_matching_bytes(&x.sub, set);
        }
        HirKind::Capture(ref x) => {
            remove_matching_bytes(&x.sub, set);
        }
        HirKind::Concat(ref xs) => {
            for x in xs {
                remove_matching_bytes(x, set);
            }
        }
        HirKind::Alternation(ref xs) => {
            for x in xs {
                remove_matching_bytes(x, set);
            }
        }
    }
}