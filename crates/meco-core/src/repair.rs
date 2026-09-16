//! Optional input repair, before source decoding.
//!
//! This is a spelling heuristic, not a morphological analyser. The small allowlist follows the
//! separated case suffixes in https://unicode.org/L2/L2019/19130-mwg3-8-mong-spec-r.pdf.
//! Forms such as bar (also "tiger") and tür (also "temporarily") are accepted only in the
//! stem contexts where a corpus shows an ordinary space before them is almost always damage.
//! See docs/suffix-separator-repair.md for the Hudum particle mapping audit: shaping support
//! alone does not establish that an ordinary space is a damaged suffix separator.

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
    "ᠢᠶᠠᠨ",
    "ᠢᠶᠡᠨ", // reflexive
    "ᠯᠤᠭ\u{180E}ᠠ",
    "ᠯᠦᠭᠡ", // comitative; preserve the internal MVS in luγ-a
    "ᠨᠤᠭᠤᠳ",
    "ᠨᠦᠭᠦᠳ",
    "ᠤᠳ",
    "ᠦᠳ", // plural
    "ᠳᠠᠭᠠᠨ",
    "ᠳᠡᠭᠡᠨ",
    "ᠲᠠᠭᠠᠨ",
    "ᠲᠡᠭᠡᠨ", // reflexive dative
    "ᠶᠤᠭᠠᠨ",
    "ᠶᠦᠭᠡᠨ", // reflexive accusative
    "ᠠᠴᠠᠭᠠᠨ",
    "ᠡᠴᠡᠭᠡᠨ", // reflexive ablative
    "ᠳᠤᠨᠢ",
    "ᠳᠦᠨᠢ",
    "ᠲᠤᠨᠢ",
    "ᠲᠦᠨᠢ", // possessive dative
    "ᠳᠠᠬᠢ",
    "ᠳᠡᠬᠢ",
    "ᠲᠠᠬᠢ",
    "ᠲᠡᠬᠢ",  // locative attributive; additions use suffix_context below
    "ᠲᠡᠬᠡᠨ", // tegen: feminine g and k share ink, and this is the common corpus spelling
    "ᠪᠠᠷ",
    "ᠪᠡᠷ", // instrumental after vowels
    "ᠪᠠᠨ",
    "ᠪᠡᠨ", // reflexive after vowels
    "ᠲᠠᠢ",
    "ᠲᠡᠢ", // comitative
    "ᠨᠠᠷ",
    "ᠨᠡᠷ", // plural
    "ᠳᠤᠭᠠᠷ",
    "ᠳᠦᠭᠡᠷ", // ordinal, after digits only
];

// Masculine and feminine spellings with identical ink. Delehi writers use either one (the
// golden corpus has mori ᠪᠡᠷ, nidü ᠪᠠᠨ, ken ᠲᠠᠢ), so only the stem's harmony is checked.
const SHARED_INK: &[&str] = &["ᠪᠠᠷ", "ᠪᠡᠷ", "ᠪᠠᠨ", "ᠪᠡᠨ", "ᠲᠠᠢ", "ᠲᠡᠢ", "ᠨᠠᠷ", "ᠨᠡᠷ"];

// Separated after a number in traditional text, including where a writer chose a plain space.
// The spelling after digits is the writer's choice, so no harmony check applies.
const AFTER_NUMBER: &[&str] = &[
    "ᠶᠢᠨ",
    "ᠤᠨ",
    "ᠦᠨ",
    "ᠤ",
    "ᠦ",
    "ᠶᠢ",
    "ᠢ",
    "ᠳᠤ",
    "ᠳᠦ",
    "ᠲᠤ",
    "ᠲᠦ",
    "ᠳᠤᠷ",
    "ᠳᠦᠷ",
    "ᠲᠤᠷ",
    "ᠲᠦᠷ",
    "ᠠᠴᠠ",
    "ᠡᠴᠡ",
    "ᠢᠶᠠᠷ",
    "ᠢᠶᠡᠷ",
    "ᠪᠠᠷ",
    "ᠪᠡᠷ",
    "ᠲᠠᠢ",
    "ᠲᠡᠢ",
    "ᠳᠠᠬᠢ",
    "ᠳᠡᠬᠢ",
    "ᠳᠤᠭᠠᠷ",
    "ᠳᠦᠭᠡᠷ",
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

// Final consonants that select the T-initial allomorph (tu, tur, taki, tegen, ...).
const T_SELECTING: &str = "ᠪᠭᠬᠷᠰᠱᠳᠲᠴᠺᠫᠹᠽᠼᠾ";

fn digit(c: char) -> bool {
    matches!(c, '0'..='9' | '\u{1810}'..='\u{1819}')
}

// A standalone number such as 25, 3.5 or ᠒᠐ ends immediately before the space. Digits glued
// to letters (MP3, a Mongolian word) are not a number context.
fn ends_with_number(before: &str) -> bool {
    let run = before.len() - before.trim_end_matches(digit).len();
    run > 0
        && before[..before.len() - run]
            .chars()
            .next_back()
            .map_or(true, |c| !c.is_alphanumeric() && !word_char(c))
}

// Audited additions use this gate. It is a filter, not a grammar checker: decline neutral-only,
// mixed-harmony and unknown control-bearing contexts. A final chachlag MVS+A/E is understood.
// In a suffix chain, `previous` is the immediately preceding segment, not the entire stem.
fn suffix_context(previous: &str, suffix: &str) -> bool {
    // T-initial allomorphs follow these consonants only. After a vowel or n, ᠲᠦᠷ is the
    // independent word tür. The comitative tai/tei follows any stem.
    if suffix.starts_with('ᠲ')
        && !matches!(suffix, "ᠲᠠᠢ" | "ᠲᠡᠢ")
        && !previous
            .chars()
            .rev()
            .find(|&c| letter(c))
            .is_some_and(|c| T_SELECTING.contains(c))
    {
        return false;
    }
    let needs_masculine = match suffix {
        "ᠢᠶᠠᠨ"
        | "ᠯᠤᠭ\u{180E}ᠠ"
        | "ᠨᠤᠭᠤᠳ"
        | "ᠤᠳ"
        | "ᠳᠠᠭᠠᠨ"
        | "ᠲᠠᠭᠠᠨ"
        | "ᠶᠤᠭᠠᠨ"
        | "ᠠᠴᠠᠭᠠᠨ"
        | "ᠳᠤᠨᠢ"
        | "ᠲᠤᠨᠢ"
        | "ᠳᠠᠬᠢ"
        | "ᠲᠠᠬᠢ" => true,
        "ᠢᠶᠡᠨ" | "ᠯᠦᠭᠡ" | "ᠨᠦᠭᠦᠳ" | "ᠦᠳ" | "ᠳᠡᠭᠡᠨ" | "ᠲᠡᠭᠡᠨ" | "ᠶᠦᠭᠡᠨ" | "ᠡᠴᠡᠭᠡᠨ" | "ᠳᠦᠨᠢ"
        | "ᠲᠦᠨᠢ" | "ᠳᠡᠬᠢ" | "ᠲᠡᠬᠢ" | "ᠲᠡᠬᠡᠨ" => false,
        "ᠳᠤᠭᠠᠷ" | "ᠳᠦᠭᠡᠷ" => return false, // dugar is also an independent word
        _ if SHARED_INK.contains(&suffix) => true,
        _ => return true,
    };
    if !previous.chars().all(letter) {
        let before_tail = previous
            .strip_suffix("\u{180E}ᠠ")
            .or_else(|| previous.strip_suffix("\u{180E}ᠡ"));
        if !before_tail.is_some_and(|stem| {
            stem.chars().all(letter) && matches!(stem.chars().last(), Some('\u{1828}'..='\u{1842}'))
        }) {
            return false;
        }
    }
    if matches!(suffix, "ᠢᠶᠠᠨ" | "ᠢᠶᠡᠨ")
        && !matches!(previous.chars().last(), Some('\u{1828}'..='\u{1842}'))
    {
        return false;
    }
    // A chachlag stem ends in A/E and counts as vowel-final.
    if matches!(suffix, "ᠪᠠᠷ" | "ᠪᠡᠷ" | "ᠪᠠᠨ" | "ᠪᠡᠨ")
        && !matches!(previous.chars().last(), Some('\u{1820}'..='\u{1827}'))
    {
        return false;
    }
    let masculine = previous.chars().any(|c| matches!(c, 'ᠠ' | 'ᠣ' | 'ᠤ'));
    let feminine = previous.chars().any(|c| matches!(c, 'ᠡ' | 'ᠧ' | 'ᠥ' | 'ᠦ'));
    if SHARED_INK.contains(&suffix) {
        masculine != feminine
    } else if needs_masculine {
        masculine && !feminine
    } else {
        feminine && !masculine
    }
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
            let previous = &input[word_start..offset];
            let word = mongolian_word(previous);
            let number = previous.is_empty() && ends_with_number(&input[..offset]);
            // Bound the lookahead to the short suffix inventory. This also avoids copying or
            // rescanning arbitrary following words in a large document.
            let suffix = (word || number)
                && SUFFIXES.iter().any(|suffix| {
                    input[next..].strip_prefix(suffix).is_some_and(|rest| {
                        (if word {
                            suffix_context(previous, suffix)
                        } else {
                            AFTER_NUMBER.contains(suffix)
                        }) && rest.chars().next().map_or(true, |c| {
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
            if suffix {
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
