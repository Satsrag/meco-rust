//! Top-level routing. Port of `service/TranslateService.java` (no Spring DI).
//!
//! Hub-and-spoke through Zvvnmod: decode `from` to the hub (unless it is the hub), then encode the
//! hub to `to` (unless it is the hub). Short-circuit exactly as Java: identity or blank input is
//! returned unchanged. Oyun and Utn57 are Unsupported (decisions); the LETTER series is Unsupported
//! until Step 6 wires Delehi/MenkLetter.

use crate::code_type::{CodeSeries, CodeType};
use crate::dispatch::{letter_from_rule, letter_to_rule, shape_from_rule, shape_to_rule};
use crate::error::MecoError;
use crate::letter::from_translator::LetterFromTranslator;
use crate::letter::to_translator::LetterToTranslator;
use crate::shape::translator::ShapeTranslator;
use crate::strings;

/// Convert `input` from one Mongolian encoding to another. UTF-8 in/out.
pub fn translate(from: CodeType, to: CodeType, input: &str) -> Result<String, MecoError> {
    if from == to || strings::is_blank(input) {
        return Ok(input.to_string());
    }
    let mut s = if from == CodeType::Zvvnmod {
        input.to_string()
    } else {
        translate_from(from, input)?
    };
    if to != CodeType::Zvvnmod {
        s = translate_to(to, &s)?;
    }
    Ok(s)
}

fn translate_from(ct: CodeType, s: &str) -> Result<String, MecoError> {
    // Oyun + Utn57 (GB/T 25914-2023) are deferred -> Unsupported (decisions #2 and 2026-06-18).
    if matches!(ct, CodeType::Oyun | CodeType::Utn57) {
        return Err(MecoError::Unsupported(ct));
    }
    match ct.code_series() {
        CodeSeries::Shape => ShapeTranslator::new(shape_from_rule(ct)?).translate(s),
        CodeSeries::Letter => LetterFromTranslator::new(letter_from_rule(ct)?).translate(s),
    }
}

fn translate_to(ct: CodeType, s: &str) -> Result<String, MecoError> {
    if matches!(ct, CodeType::Oyun | CodeType::Utn57) {
        return Err(MecoError::Unsupported(ct));
    }
    match ct.code_series() {
        CodeSeries::Shape => ShapeTranslator::new(shape_to_rule(ct)?).translate(s),
        CodeSeries::Letter => LetterToTranslator::new(letter_to_rule(ct)?).translate(s),
    }
}
