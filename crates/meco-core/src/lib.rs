#![forbid(unsafe_code)]
// TODO(remove before binding stage): silence dead_code while the spine is wired up incrementally.
// Every item is exercised by unit tests; once the router/translators consume them this comes off.
#![allow(dead_code)]
//! # meco-core
//!
//! Pure-Rust core of the Mongolian Encoding Converter (`meco`), ported from the
//! original Java library `com.zvvnmod.meco`.
//!
//! Design & decisions: `docs/superpowers/specs/2026-06-18-meco-rust-port-design.md`.
//!
//! Ground rules baked into this crate:
//! - Generated Java tables are the historical behavior baseline; Rust-owned Zvvnmod rules may
//!   intentionally override them when the project adopts corrected semantics.
//! - Conversions route through the intermediate **Zvvnmod** encoding.
//! - The default build has no I/O or framework and is pure compute. The explicit
//!   `utn57-command` feature adds command-backed UTN #57 target conversion for servers/desktops.
//!
//! Status: scaffolding — the shared spine (encoding types, errors, string helpers) is in place.
//! Translation routing and the shape/letter subsystems are added in later steps.

mod code_mapper;
mod code_type;
mod dispatch;
mod error;
mod letter;
mod router;
mod shape;
mod strings;
mod tables;
mod unicode;
mod word;

pub use code_type::{CodeSeries, CodeType};
pub use error::MecoError;
pub use router::translate;

/// Crate version (currently just the Cargo package version). A table-provenance tag
/// (the Java commit the generated tables come from) will be appended once tables exist.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
