#![forbid(unsafe_code)]

//! Opt-in command-backed UTN #57 conversion for `meco`.

use meco_core::{CodeType, MecoError};
use std::error::Error;
use std::fmt;
use zvvnmod_utn57::{convert_zvvnmod_to_utn57, Utn57TextConversionError};

/// Stable classification of an optional UTN #57 adapter failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MecoUtn57CommandErrorKind {
    /// The portable meco core rejected the source or non-UTN57 target conversion.
    Core,
    /// Complete-text ZVVNMOD → UTN #57 conversion failed.
    Utn57,
    /// Canonical UTN #57 text cannot be converted back to another encoding.
    UnsupportedDirection { from: CodeType, to: CodeType },
}

#[derive(Debug)]
enum ErrorSource {
    Core(MecoError),
    Utn57(Utn57TextConversionError),
}

/// Failure while routing a conversion through the optional UTN #57 adapter.
///
/// Backend-specific error types stay private; callers can branch on [`Self::kind`]
/// and inspect the standard [`Error::source`] chain for diagnostics.
#[derive(Debug)]
pub struct MecoUtn57CommandError {
    kind: MecoUtn57CommandErrorKind,
    source: Option<ErrorSource>,
}

impl MecoUtn57CommandError {
    /// Return the stable, backend-neutral error classification.
    pub const fn kind(&self) -> MecoUtn57CommandErrorKind {
        self.kind
    }

    fn unsupported_direction(from: CodeType, to: CodeType) -> Self {
        Self {
            kind: MecoUtn57CommandErrorKind::UnsupportedDirection { from, to },
            source: None,
        }
    }
}

impl fmt::Display for MecoUtn57CommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            Some(ErrorSource::Core(error)) => error.fmt(formatter),
            Some(ErrorSource::Utn57(error)) => error.fmt(formatter),
            None => match self.kind {
                MecoUtn57CommandErrorKind::UnsupportedDirection { from, to } => {
                    write!(
                        formatter,
                        "conversion not supported from {from:?} to {to:?}"
                    )
                }
                MecoUtn57CommandErrorKind::Core | MecoUtn57CommandErrorKind::Utn57 => {
                    formatter.write_str("conversion failed without an error source")
                }
            },
        }
    }
}

impl Error for MecoUtn57CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.source.as_ref() {
            Some(ErrorSource::Core(error)) => Some(error),
            Some(ErrorSource::Utn57(error)) => Some(error),
            None => None,
        }
    }
}

impl From<MecoError> for MecoUtn57CommandError {
    fn from(source: MecoError) -> Self {
        Self {
            kind: MecoUtn57CommandErrorKind::Core,
            source: Some(ErrorSource::Core(source)),
        }
    }
}

impl From<Utn57TextConversionError> for MecoUtn57CommandError {
    fn from(source: Utn57TextConversionError) -> Self {
        Self {
            kind: MecoUtn57CommandErrorKind::Utn57,
            source: Some(ErrorSource::Utn57(source)),
        }
    }
}

/// Convert between meco encodings, including the opt-in command-backed UTN #57 target.
///
/// Non-UTN57 routes delegate unchanged to [`meco_core::translate`]. A UTN #57 target
/// first converts the declared source to ZVVNMOD in the portable core and then calls
/// `zvvnmod-utn57`'s complete-text facade. Reverse conversion from UTN #57 remains
/// unsupported. Identity conversion never starts the external backend.
pub fn translate(
    from: CodeType,
    to: CodeType,
    input: &str,
) -> Result<String, MecoUtn57CommandError> {
    if from != CodeType::Utn57 && to != CodeType::Utn57 {
        return meco_core::translate(from, to, input).map_err(Into::into);
    }
    if from == to {
        return Ok(input.to_owned());
    }
    if to == CodeType::Utn57 {
        let zvvnmod = if from == CodeType::Zvvnmod {
            input.to_owned()
        } else {
            meco_core::translate(from, CodeType::Zvvnmod, input)?
        };
        return convert_zvvnmod_to_utn57(&zvvnmod).map_err(Into::into);
    }

    Err(MecoUtn57CommandError::unsupported_direction(from, to))
}
