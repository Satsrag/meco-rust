//! Top-level routing. Port of `service/TranslateService.java` (no Spring DI).
//!
//! Hub-and-spoke through Zvvnmod: decode `from` to the hub (unless it is the hub), then encode the
//! hub to `to` (unless it is the hub). Short-circuit exactly as Java: identity or blank input is
//! returned unchanged. Oyun stays Unsupported in both directions. UTN #57 goes both ways, handed
//! to the pure-Rust `zvvnmod-utn57` crate in process rather than through the letter/shape tables.

use crate::code_type::{CodeSeries, CodeType};
use crate::dispatch::{letter_from_rule, letter_to_rule, shape_from_rule, shape_to_rule};
use crate::error::MecoError;
use crate::letter::from_translator::LetterFromTranslator;
use crate::letter::rule::WORD_CONNECTOR;
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
    if ct == CodeType::Oyun {
        return Err(MecoError::Unsupported(ct));
    }
    if ct == CodeType::Utn57 {
        return zvvnmod_utn57::convert_utn57_to_zvvnmod(s)
            .map_err(|error| MecoError::Utn57(error.to_string()));
    }
    match ct.code_series() {
        CodeSeries::Shape => ShapeTranslator::new(shape_from_rule(ct)?).translate(s),
        CodeSeries::Letter => LetterFromTranslator::new(letter_from_rule(ct)?).translate(s),
    }
}

fn translate_to(ct: CodeType, s: &str) -> Result<String, MecoError> {
    if ct == CodeType::Oyun {
        return Err(MecoError::Unsupported(ct));
    }
    if ct == CodeType::Utn57 {
        return zvvnmod_utn57::convert_zvvnmod_to_utn57(s)
            .map_err(|error| MecoError::Utn57(error.to_string()));
    }
    match ct.code_series() {
        // The shape encodings have no NNBSP of their own: they spell a suffix boundary with an
        // ordinary space and always have, so the hub's connector is flattened back for them.
        CodeSeries::Shape => {
            let flattened = s.replace(WORD_CONNECTOR, " ");
            ShapeTranslator::new(shape_to_rule(ct)?).translate(&flattened)
        }
        CodeSeries::Letter => LetterToTranslator::new(letter_to_rule(ct)?).translate(s),
    }
}
