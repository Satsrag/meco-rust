//! Error type. Folds the Java `State` / `TranslateState` / `DelehiState` runtime codes into one
//! Rust enum.
//!
//! Note (design decision #3): a content-level *unmappable code point* is **not** an error here —
//! it is passed through unchanged. So [`MecoError::NotFoundInMapper`] is reserved for internal/
//! diagnostic use. The default build's public `translate` returns `Err` only for structural problems
//! (unsupported encoding, unsupported series, unknown enum string). With `utn57-command`, backend
//! failures are also returned through this same error channel.

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
    /// Conversion involving this code is not supported in the active build.
    Unsupported(CodeType),
    /// The optional command-backed UTN #57 target conversion failed.
    #[cfg(feature = "utn57-command")]
    Utn57(String),
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
            #[cfg(feature = "utn57-command")]
            MecoError::Utn57(reason) => write!(f, "UTN #57 conversion failed: {reason}"),
        }
    }
}

impl std::error::Error for MecoError {}
