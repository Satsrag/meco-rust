//! Ports of the `LetterTranslateRuleFrom` / `LetterTranslateRuleTo` Java interfaces.

use crate::error::MecoError;
use crate::word::nature::Nature;
use crate::word::shape_word::ShapeWord;

pub(crate) trait LetterTranslateRuleFrom: Sync {
    /// Map a committed fragment to Zvvnmod, given preceding/following fragment content and the
    /// resolved nature. `None` means the key is absent.
    fn get_mapper_code(
        &self,
        pre: &[char],
        suf: &[char],
        key: &str,
        nature: Nature,
    ) -> Option<&'static str>;

    fn contains(&self, key: &str) -> bool;

    fn get_code_nature(&self, c: char) -> Nature;

    /// Participates in conversion at all (e.g. includes the word connector U+202F).
    fn is_translate_code_point(&self, c: char) -> bool;

    /// Triggers greedy word matching (the connector is translate-but-not-word).
    fn is_word_code_point(&self, c: char) -> bool;
}

pub(crate) trait LetterTranslateRuleTo: Sync {
    /// Emit the whole Zvvnmod word into `builder` (the to-direction rule owns word-level state like
    /// cross-fragment merges and chagh/hundii fallback).
    fn get_mapper_code(&self, builder: &mut String, word: &ShapeWord) -> Result<(), MecoError>;

    fn contains(&self, key: &str) -> bool;

    fn is_translate_code_point(&self, c: char) -> bool;
}
