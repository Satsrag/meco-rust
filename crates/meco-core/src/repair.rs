//! Optional input repair, before source decoding.
//!
//! This is a spelling heuristic, not a morphological analyser. The small allowlist follows the
//! separated case suffixes in https://unicode.org/L2/L2019/19130-mwg3-8-mong-spec-r.pdf.
//! Ambiguous standalone forms such as bar (also "tiger") and tai are deliberately excluded.

use crate::{CodeType, MecoError, Warning};
use std::borrow::Cow;

// MenkLetter/Delehi letter spellings, not canonical UTN #57 spellings. No FVS is stripped to
// make a match: a selector may carry information this heuristic does not understand.
const SUFFIXES: &[&str] = &[
    "ᠶᠢᠨ",
    "ᠤᠨ",
    "ᠦᠨ",
    "ᠤ",
    "ᠦ", // genitive
    "ᠶᠢ",
    "ᠢ", // accusative
    "ᠳᠤ",
    "ᠳᠦ",
    "ᠲᠤ",
    "ᠲᠦ",
    "ᠳᠤᠷ",
    "ᠳᠦᠷ",
    "ᠲᠤᠷ",
    "ᠲᠦᠷ", // dative
    "ᠠᠴᠠ",
    "ᠡᠴᠡ", // ablative
    "ᠢᠶᠠᠷ",
    "ᠢᠶᠡᠷ", // instrumental (bar/ber excluded)
];

fn letter(c: char) -> bool {
    matches!(c, '\u{1820}'..='\u{1842}')
}

fn word_char(c: char) -> bool {
    letter(c) || matches!(c, '\u{180B}'..='\u{180F}' | '\u{200C}' | '\u{200D}')
}

fn mongolian_word(s: &str) -> bool {
    s.chars().all(word_char) && s.chars().any(letter)
}

/// Replace a single ordinary or non-breaking space before an allowlisted suffix. All offsets
/// describe the original UTF-8 input. Existing suffix boundaries, layout and word letters stay
/// untouched. Runs joined without a space are never split.
pub(crate) fn suffix_separators(
    from: CodeType,
    input: &str,
) -> Result<(Cow<'_, str>, Vec<Warning>), MecoError> {
    if !matches!(from, CodeType::MenkLetter | CodeType::Delehi) {
        return Err(MecoError::UnsupportedInputRepair(from));
    }

    let mut out = String::new();
    let mut warnings = Vec::new();
    let mut copied = 0;
    let mut word_start = 0;
    for (offset, c) in input.char_indices() {
        if matches!(c, ' ' | '\u{00A0}') {
            let next = offset + c.len_utf8();
            // Bound the lookahead to the short suffix inventory. This also avoids copying or
            // rescanning arbitrary following words in a large document.
            let suffix = SUFFIXES.iter().any(|suffix| {
                input[next..].strip_prefix(suffix).is_some_and(|rest| {
                    rest.chars().next().map_or(true, |c| {
                        c.is_whitespace()
                            || matches!(
                                c,
                                '\u{1800}'
                                    ..='\u{1809}'
                                        | '.'
                                        | ','
                                        | ';'
                                        | ':'
                                        | '!'
                                        | '?'
                                        | ')'
                                        | ']'
                                        | '}'
                                        | '"'
                                        | '\''
                            )
                    })
                })
            });
            if suffix && mongolian_word(&input[word_start..offset]) {
                out.push_str(&input[copied..offset]);
                out.push('\u{202F}');
                copied = next;
                warnings.push(Warning::RepairedSuffixSeparator {
                    byte_offset: offset,
                    original: c,
                });
            }
        }
        if !word_char(c) {
            word_start = offset + c.len_utf8();
        }
    }
    if warnings.is_empty() {
        Ok((Cow::Borrowed(input), warnings))
    } else {
        out.push_str(&input[copied..]);
        Ok((Cow::Owned(out), warnings))
    }
}
