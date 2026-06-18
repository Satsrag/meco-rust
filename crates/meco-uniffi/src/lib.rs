//! UniFFI binding for `meco-core`: generates idiomatic Swift (iOS) and Kotlin (Android) bindings
//! from this one Rust crate, using proc-macro mode (no `.udl`). `meco-core` stays dependency-free
//! and `#![forbid(unsafe_code)]`; UniFFI's generated scaffolding lives here.

uniffi::setup_scaffolding!();

use meco_core::CodeType;
use std::str::FromStr;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum MecoError {
    #[error("{message}")]
    Translate { message: String },
}

/// Translate `input` from encoding `from` to `to`. `from`/`to` are canonical encoding names
/// ("zvvnmod", "delehi", "menk_shape", "menk_letter", "z52"). Throws on an unknown encoding or an
/// unsupported conversion. UTF-8 throughout (Swift `String` / Kotlin `String`).
#[uniffi::export]
pub fn translate(from: String, to: String, input: String) -> Result<String, MecoError> {
    let parse = |s: &str| {
        CodeType::from_str(s).map_err(|e| MecoError::Translate { message: e.to_string() })
    };
    let from = parse(&from)?;
    let to = parse(&to)?;
    meco_core::translate(from, to, &input).map_err(|e| MecoError::Translate { message: e.to_string() })
}

/// Library version.
#[uniffi::export]
pub fn version() -> String {
    meco_core::version().to_string()
}
