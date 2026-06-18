//! WebAssembly binding for `meco-core` via wasm-bindgen — an npm package usable in the browser,
//! Node, Deno/Bun and edge runtimes (Cloudflare Workers, etc.). Strings marshal automatically.
//! `meco-core` stays `#![forbid(unsafe_code)]`; wasm-bindgen's glue is confined to this crate.

use meco_core::CodeType;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// Translate `input` from encoding `from` to `to`. `from`/`to` are canonical encoding names
/// ("zvvnmod", "delehi", "menk_shape", "menk_letter", "z52"). Throws a JS `Error` on an unknown
/// encoding name or an unsupported conversion.
#[wasm_bindgen]
pub fn translate(from: &str, to: &str, input: &str) -> Result<String, JsError> {
    let from = CodeType::from_str(from).map_err(|e| JsError::new(&e.to_string()))?;
    let to = CodeType::from_str(to).map_err(|e| JsError::new(&e.to_string()))?;
    meco_core::translate(from, to, input).map_err(|e| JsError::new(&e.to_string()))
}

/// Library version.
#[wasm_bindgen]
pub fn version() -> String {
    meco_core::version().to_string()
}
