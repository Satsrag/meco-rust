//! Port of `letter/from/LetterFromTranslator.java`. Letter -> Zvvnmod.
//!
//! Like the shape translator but over `LetterWord`/`LetterWordFragment`, with two distinctions:
//! `is_word_code_point` (greedy trigger) vs `is_translate_code_point` (a connector attaches to the
//! next fragment), and per-fragment Nature that resolves against the word nature in `translate_word`.

use crate::error::MecoError;
use crate::letter::rule::LetterTranslateRuleFrom;
use crate::strings;
use crate::word::char_type::CharType;
use crate::word::letter_word::{LetterWord, LetterWordFragment};
use crate::word::nature::Nature;

pub(crate) struct LetterFromTranslator {
    rule: &'static dyn LetterTranslateRuleFrom,
}

impl LetterFromTranslator {
    pub fn new(rule: &'static dyn LetterTranslateRuleFrom) -> Self {
        Self { rule }
    }

    fn unicode_type(&self, c: char) -> CharType {
        if self.rule.is_word_code_point(c) {
            CharType::Mongolian
        } else {
            CharType::Other
        }
    }

    pub fn translate(&self, s0: &str) -> Result<String, MecoError> {
        if strings::is_blank(s0) {
            return Ok(String::new());
        }
        let mut chars: Vec<char> = s0.chars().collect();
        chars.push('\u{e666}');

        let mut builder = String::new();
        let mut fragment = LetterWordFragment::new();
        let mut word = LetterWord::new();
        fragment.set_head(Some(CharType::Other));

        let mut i = 0usize;
        while i < chars.len() {
            let c = chars[i];
            if self.rule.is_word_code_point(c) {
                fragment.push(c);
                fragment.set_tail(Some(self.unicode_type(chars[i + 1])));
                if self.rule.contains(&fragment.get_key()) {
                    fragment.set_nature(self.rule.get_code_nature(c));
                    i += 1;
                    continue;
                }
                fragment.pop()?;
                fragment.set_tail(Some(self.unicode_type(c)));
                if fragment.is_blank() {
                    // Passthrough (decision #3): in-range but unmappable -> emit verbatim.
                    if word.is_not_blank() {
                        self.translate_word(&mut builder, &mut word)?;
                        word = LetterWord::new();
                    }
                    builder.push(c);
                    fragment = LetterWordFragment::new();
                    fragment.set_head(Some(CharType::Other));
                    i += 1;
                    continue;
                }
                word.add(fragment);
                fragment = LetterWordFragment::new();
                fragment.set_head(Some(self.unicode_type(chars[i - 1])));
                continue; // reprocess c
            } else {
                if fragment.is_not_blank() {
                    fragment.set_tail(Some(CharType::Other));
                    word.add(fragment);
                    fragment = LetterWordFragment::new();
                    fragment.set_head(Some(CharType::Other));
                }
                if word.is_not_blank() {
                    self.translate_word(&mut builder, &mut word)?;
                    word = LetterWord::new();
                }
                if self.rule.is_translate_code_point(c) {
                    fragment.push(c);
                } else {
                    builder.push(c);
                }
            }
            i += 1;
        }
        builder.pop(); // strip sentinel
        Ok(builder)
    }

    fn translate_word(&self, builder: &mut String, word: &mut LetterWord) -> Result<(), MecoError> {
        word.remove_invalid_code_point_from_end();
        let word_nature = word.nature;
        let n = word.fragments.len();
        let mut pre: Vec<char> = Vec::new();
        for idx in 0..n {
            let wf = &word.fragments[idx];
            let nature = if wf.nature() == Nature::Saarmag { word_nature } else { wf.nature() };
            let suf: &[char] = if idx + 1 < n {
                word.fragments[idx + 1].content()
            } else {
                &[]
            };
            match self.rule.get_mapper_code(&pre, suf, &wf.get_key(), nature) {
                Some(s) => builder.push_str(s),
                None => return Err(MecoError::NotFoundInMapper(wf.get_key())),
            }
            pre.extend(wf.content());
        }
        Ok(())
    }
}
