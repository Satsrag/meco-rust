//! WebAssembly binding for `meco-core` via wasm-bindgen — an npm package usable in the browser,
//! Node, Deno/Bun and edge runtimes (Cloudflare Workers, etc.). Strings marshal automatically.
//! `meco-core` stays `#![forbid(unsafe_code)]`; wasm-bindgen's glue is confined to this crate.

use meco_core::CodeType;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// Translate `input` from encoding `from` to `to`. `from`/`to` are canonical encoding names
/// ("zvvnmod", "delehi", "menk_shape", "menk_letter", "z52", "utn57").
/// Throws a JS `Error` on an unknown encoding name or an unsupported conversion.
#[wasm_bindgen]
pub fn translate(from: &str, to: &str, input: &str) -> Result<String, JsError> {
    let from = CodeType::from_str(from).map_err(|e| JsError::new(&e.to_string()))?;
    let to = CodeType::from_str(to).map_err(|e| JsError::new(&e.to_string()))?;
    meco_core::translate(from, to, input).map_err(|e| JsError::new(&e.to_string()))
}

/// A finished conversion: `text` is what `translate` returns, `warnings` says what the conversion
/// had to do beyond what the input said (empty for most conversions). Today only the `utn57`
/// target raises any, for a hub run it could spell only with an invented ZWJ.
#[wasm_bindgen(getter_with_clone)]
pub struct Translation {
    pub text: String,
    pub warnings: Vec<String>,
}

/// Like `translate`, and also reports the conversion's warnings. Throws on the same errors.
#[wasm_bindgen]
pub fn translate_with_warnings(from: &str, to: &str, input: &str) -> Result<Translation, JsError> {
    let from = CodeType::from_str(from).map_err(|e| JsError::new(&e.to_string()))?;
    let to = CodeType::from_str(to).map_err(|e| JsError::new(&e.to_string()))?;
    let translation = meco_core::translate_with_warnings(from, to, input)
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(Translation {
        text: translation.text,
        warnings: translation
            .warnings
            .iter()
            .map(|warning| warning.to_string())
            .collect(),
    })
}

/// Library version.
#[wasm_bindgen]
pub fn version() -> String {
    meco_core::version().to_string()
}
