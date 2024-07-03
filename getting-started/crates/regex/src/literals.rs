use {
    regex_automata::meta::Regex,
    regex_syntax::hir::{
        self,
        literal::{Literal, Seq},
        Hir,
    },
};

use crate::{config::ConfiguredHIR, error::Error};

#[derive(Clone, Debug)]
pub(crate) struct InnerLiterals {
    seq: Seq,
}

impl InnerLiterals {
    /// Create a set of inner literals from the given HIR expression.
    ///
    /// If no line terminator was configured, then this always declines to
    /// extract literals because the inner literal optimization may not be
    /// valid.
    ///
    /// Note that this requires the actual regex that will be used for a search
    /// because it will query some state about the compiled regex. That state
    /// may influence inner literal extraction.
    pub(crate) fn new(chir: &ConfiguredHIR, re: &Regex) -> InnerLiterals {
        // If there's no line terminator, then the inner literal optimization
        // at this level is not valid.
        if chir.config().line_terminator.is_none() {
            log::trace!(
                "skipping inner literal extraction, \
                 no line terminator is set"
            );
            return InnerLiterals::none();
        }
        // If we believe the regex is already accelerated, then just let
        // the regex engine do its thing. We'll skip the inner literal
        // optimization.
        //
        // ... but only if the regex doesn't have any Unicode word boundaries.
        // If it does, there's enough of a chance of the regex engine falling
        // back to a slower engine that it's worth trying our own inner literal
        // optimization.
        if re.is_accelerated() {
            if !chir.hir().properties().look_set().contains_word_unicode() {
                log::trace!(
                    "skipping inner literal extraction, \
                     existing regex is believed to already be accelerated",
                );
                return InnerLiterals::none();
            }
        }
        // In this case, we pretty much know that the regex engine will handle
        // it as best as possible, even if it isn't reported as accelerated.
        if chir.hir().properties().is_alternation_literal() {
            log::trace!(
                "skipping inner literal extraction, \
                 found alternation of literals, deferring to regex engine",
            );
            return InnerLiterals::none();
        }
        let seq = Extractor::new().extract_untagged(chir.hir());
        InnerLiterals { seq }
    }

    /// Returns a infinite set of inner literals, such that it can never
    /// produce a matcher.
    pub(crate) fn none() -> InnerLiterals {
        InnerLiterals { seq: Seq::infinite() }
    }

    pub(crate) fn one_regex(&self) -> Result<Option<Regex>, Error> {
        let Some(lits) = self.seq.literals() else { return Ok(None) };
        if lits.is_empty() {
            return Ok(None);
        }
        let mut alts = vec![];
        for lit in lits.iter() {
            alts.push(Hir::literal(lit.as_bytes()));
        }
        let hir = Hir::alternation(alts);
        log::debug!("extracted fast line regex: {:?}", hir.to_string());
        let re = Regex::builder()
            .configure(Regex::config().utf8_empty(false))
            .build_from_hir(&hir)
            .map_err(Error::regex)?;
        Ok(Some(re))
    }
}

#[derive(Debug)]
struct Extractor {
    limit_class: usize,
    limit_repeat: usize,
    limit_literal_len: usize,
    limit_total: usize,
}

impl Extractor {
    /// Create a new inner literal extractor with a default configuration.
    fn new() -> Extractor {
        Extractor {
            limit_class: 10,
            limit_repeat: 10,
            limit_literal_len: 100,
            limit_total: 64,
        }
    }

    /// Execute the extractor at the top-level and return an untagged sequence
    /// of literals.
    fn extract_untagged(&self, hir: &Hir) -> Seq {
        let mut seq = self.extract(hir);
        log::trace!("extracted inner literals: {:?}", seq.seq);
        seq.seq.optimize_for_prefix_by_preference();
        log::trace!(
            "extracted inner literals after optimization: {:?}",
            seq.seq
        );
        if !seq.is_good() {
            log::trace!(
                "throwing away inner literals because they might be slow"
            );
            seq.make_infinite();
        }
        seq.seq
    }

    fn extract(&self, hir: &Hir) -> TSeq {
        use regex_syntax::hir::HirKind::*;

        match *hir.kind() {
            Empty | Look(_) => TSeq::singleton(self::Literal::exact(vec![])),
            Literal(hir::Literal(ref bytes)) => {
                let mut seq =
                    TSeq::singleton(self::Literal::exact(bytes.to_vec()));
                self.enforce_literal_len(&mut seq);
                seq
            }
            Class(hir::Class::Unicode(ref cls)) => {
                self.extract_class_unicode(cls)
            }
            Class(hir::Class::Bytes(ref cls)) => self.extract_class_bytes(cls),
            Repetition(ref rep) => self.extract_repetition(rep),
            Capture(hir::Capture { ref sub, .. }) => self.extract(sub),
            Concat(ref hirs) => self.extract_concat(hirs.iter()),
            Alternation(ref hirs) => self.extract_alternation(hirs.iter()),
        }
    }
}

#[derive(Clone, Debug)]
struct TSeq {
    seq: Seq,
    prefix: bool,
}

#[allow(dead_code)]
impl TSeq {
    fn empty() -> TSeq {
        TSeq { seq: Seq::empty(), prefix: true }
    }

    fn infinite() -> TSeq {
        TSeq { seq: Seq::infinite(), prefix: true }
    }

    fn singleton(lit: Literal) -> TSeq {
        TSeq { seq: Seq::singleton(lit), prefix: true }
    }

    fn new<I, B>(it: I) -> TSeq
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        TSeq { seq: Seq::new(it), prefix: true }
    }

    fn literals(&self) -> Option<&[Literal]> {
        self.seq.literals()
    }

    fn push(&mut self, lit: Literal) {
        self.seq.push(lit);
    }

    fn make_inexact(&mut self) {
        self.seq.make_inexact();
    }

    fn make_infinite(&mut self) {
        self.seq.make_infinite();
    }

    fn cross_forward(&mut self, other: &mut TSeq) {
        assert!(other.prefix);
        self.seq.cross_forward(&mut other.seq);
    }

    fn union(&mut self, other: &mut TSeq) {
        self.seq.union(&mut other.seq);
    }

    fn dedup(&mut self) {
        self.seq.dedup();
    }

    fn sort(&mut self) {
        self.seq.sort();
    }

    fn keep_first_bytes(&mut self, len: usize) {
        self.seq.keep_first_bytes(len);
    }

    fn is_finite(&self) -> bool {
        self.seq.is_finite()
    }

    fn is_empty(&self) -> bool {
        self.seq.is_empty()
    }

    fn len(&self) -> Option<usize> {
        self.seq.len()
    }

    fn is_exact(&self) -> bool {
        self.seq.is_exact()
    }

    fn is_inexact(&self) -> bool {
        self.seq.is_inexact()
    }

    fn max_union_len(&self, other: &TSeq) -> Option<usize> {
        self.seq.max_union_len(&other.seq)
    }

    fn max_cross_len(&self, other: &TSeq) -> Option<usize> {
        assert!(other.prefix);
        self.seq.max_cross_len(&other.seq)
    }

    fn min_literal_len(&self) -> Option<usize> {
        self.seq.min_literal_len()
    }

    fn max_literal_len(&self) -> Option<usize> {
        self.seq.max_literal_len()
    }

    // Below are methods specific to a TSeq that aren't just forwarding calls
    // to a Seq method.

    /// Tags this sequence as "not a prefix." When this happens, this sequence
    /// can't be crossed as a suffix of another sequence.
    fn make_not_prefix(&mut self) {
        self.prefix = false;
    }

    /// Returns true if it's believed that the sequence given is "good" for
    /// acceleration. This is useful for determining whether a sequence of
    /// literals has any shot of being fast.
    fn is_good(&self) -> bool {
        if self.has_poisonous_literal() {
            return false;
        }
        let Some(min) = self.min_literal_len() else { return false };
        let Some(len) = self.len() else { return false };
        // If we have some very short literals, then let's require that our
        // sequence is itself very small.
        if min <= 1 {
            return len <= 3;
        }
        min >= 2 && len <= 64
    }

    /// Returns true if it's believed that the sequence given is "really
    /// good" for acceleration. This is useful for short circuiting literal
    /// extraction.
    fn is_really_good(&self) -> bool {
        if self.has_poisonous_literal() {
            return false;
        }
        let Some(min) = self.min_literal_len() else { return false };
        let Some(len) = self.len() else { return false };
        min >= 3 && len <= 8
    }

    /// Returns true if the given sequence contains a poisonous literal.
    fn has_poisonous_literal(&self) -> bool {
        let Some(lits) = self.literals() else { return false };
        lits.iter().any(is_poisonous)
    }

    /// Compare the two sequences and return the one that is believed to be best
    /// according to a hodge podge of heuristics.
    fn choose(self, other: TSeq) -> TSeq {
        let (seq1, seq2) = (self, other);
        if !seq1.is_finite() {
            return seq2;
        } else if !seq2.is_finite() {
            return seq1;
        }
        if seq1.has_poisonous_literal() {
            return seq2;
        } else if seq2.has_poisonous_literal() {
            return seq1;
        }
        let Some(min1) = seq1.min_literal_len() else { return seq2 };
        let Some(min2) = seq2.min_literal_len() else { return seq1 };
        if min1 < min2 {
            return seq2;
        } else if min2 < min1 {
            return seq1;
        }
        // OK because we know both sequences are finite, otherwise they wouldn't
        // have a minimum literal length.
        let len1 = seq1.len().unwrap();
        let len2 = seq2.len().unwrap();
        if len1 < len2 {
            return seq2;
        } else if len2 < len1 {
            return seq1;
        }
        // We could do extra stuff like looking at a background frequency
        // distribution of bytes and picking the one that looks more rare, but for
        // now we just pick one.
        seq1
    }
}

impl FromIterator<Literal> for TSeq {
    fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> TSeq {
        TSeq { seq: Seq::from_iter(it), prefix: true }
    }
}

/// Returns true if it is believe that this literal is likely to match very
/// frequently, and is thus not a good candidate for a prefilter.
fn is_poisonous(lit: &Literal) -> bool {
    use regex_syntax::hir::literal::rank;

    lit.is_empty() || (lit.len() == 1 && rank(lit.as_bytes()[0]) >= 250)
}