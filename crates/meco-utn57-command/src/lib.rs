//! Desktop/server adapter from meco encodings to canonical UTN #57.
//!
//! This crate intentionally keeps the external normalization command out of
//! `meco-core` and the portable WASM, UniFFI, and C ABI packages.

use meco_core::{translate, CodeType, MecoError};
use std::error::Error;
use std::fmt;
use zvvnmod_utn57::{convert_zvvnmod_to_utn57, Utn57TextConversionError};

/// Failure while routing a meco encoding through ZVVNMOD to UTN #57.
#[derive(Debug)]
pub enum MecoUtn57Error {
    /// The source encoding could not be converted to the ZVVNMOD hub.
    Meco(MecoError),
    /// The ZVVNMOD text could not be normalized to canonical UTN #57.
    Utn57(Utn57TextConversionError),
}

impl fmt::Display for MecoUtn57Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Meco(error) => error.fmt(formatter),
            Self::Utn57(error) => error.fmt(formatter),
        }
    }
}

impl Error for MecoUtn57Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Meco(error) => Some(error),
            Self::Utn57(error) => Some(error),
        }
    }
}

impl From<MecoError> for MecoUtn57Error {
    fn from(error: MecoError) -> Self {
        Self::Meco(error)
    }
}

impl From<Utn57TextConversionError> for MecoUtn57Error {
    fn from(error: Utn57TextConversionError) -> Self {
        Self::Utn57(error)
    }
}

/// Convert a supported meco source encoding to canonical UTN #57.
///
/// The portable core first converts `input` to the ZVVNMOD hub. The
/// backend-neutral `zvvnmod-utn57` facade then performs final normalization.
pub fn translate_to_utn57(from: CodeType, input: &str) -> Result<String, MecoUtn57Error> {
    let zvvnmod = translate(from, CodeType::Zvvnmod, input)?;
    Ok(convert_zvvnmod_to_utn57(&zvvnmod)?)
}
