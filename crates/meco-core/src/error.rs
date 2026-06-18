//! Error type. Folds the Java `State` / `TranslateState` / `DelehiState` runtime codes into one
//! Rust enum.
//!
//! Note (design decision #3): a content-level *unmappable code point* is **not** an error here —
//! it is passed through unchanged. So [`MecoError::NotFoundInMapper`] is reserved for internal/
//! diagnostic use; the public `translate` returns `Err` only for structural problems
//! (unsupported encoding, unsupported series, unknown enum string).

use crate::code_type::CodeType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MecoError {
    /// No translate rule registered for this code (defensive; should be unreachable for supported types).
    MissTranslateRule(CodeType),
    /// Internal stack underflow during fragment processing.
    NothingToPop,
    /// A key was not found in a mapper table (internal/diagnostic; content path passes through instead).
    NotFoundInMapper(String),
    /// A code's series was neither Letter nor Shape (defensive; unreachable given the enum).
    NotSupportedCodeSeries(CodeType),
    /// A string could not be parsed into a [`CodeType`].
    UnsupportedEnumType(String),
    /// Conversion involving this code is not (yet) supported — currently `Oyun` and `Utn57`.
    Unsupported(CodeType),
}

impl fmt::Display for MecoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MecoError::MissTranslateRule(ct) => write!(f, "missing translate rule for {ct:?}"),
            MecoError::NothingToPop => write!(f, "nothing to pop"),
            MecoError::NotFoundInMapper(k) => write!(f, "key not found in mapper: {k:?}"),
            MecoError::NotSupportedCodeSeries(ct) => write!(f, "unsupported code series for {ct:?}"),
            MecoError::UnsupportedEnumType(s) => write!(f, "unsupported encoding name: {s:?}"),
            MecoError::Unsupported(ct) => write!(f, "conversion not supported for {ct:?}"),
        }
    }
}

impl std::error::Error for MecoError {}
