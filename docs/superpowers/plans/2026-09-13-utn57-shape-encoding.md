# `utn57_shape` Encoding Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the `utn57_shape` encoding — a UTN #57 word spelled as its written units (`SAIIA`) — reading and writing through the existing `utn57` path.

**Architecture:** `utn57_shape` is `utn57` seen through mongol-norm's shape function. A small codec module turns UTN #57 text into shape text (`Shaper::shape` per word, compact PascalCase) and shape text into UTN #57 text (`Shaper::parse_written_units` + `normalize_written_units` per word). The router decodes before, or encodes after, the existing `utn57` conversion, so the hub and every other encoding are untouched.

**Tech Stack:** Rust; `mongol_norm` reached as `zvvnmod_utn57::mongol_norm` (already a dependency); wasm-bindgen page in `crates/meco-wasm/web/index.html`.

Spec: `docs/superpowers/specs/2026-09-13-utn57-shape-encoding-design.md`.

---

### Task 1: The `Utn57Shape` code type

**Files:**
- Modify: `crates/meco-core/src/code_type.rs`

- [ ] **Step 1: Write the failing test** (append to the `tests` module at the bottom of `code_type.rs`)

```rust
    #[test]
    fn utn57_shape_parses_and_prints_like_menk_shape() {
        assert_eq!(CodeType::get("utn57_shape").unwrap(), CodeType::Utn57Shape);
        assert_eq!(CodeType::get("UTN57SHAPE").unwrap(), CodeType::Utn57Shape);
        assert_eq!(CodeType::Utn57Shape.canonical_str(), "utn57_shape");
        assert_eq!(CodeType::Utn57Shape.code_series(), CodeSeries::Letter);
    }
```

- [ ] **Step 2: Run it** — `cargo test -q -p meco-core --lib code_type` — expected: compile error, no variant `Utn57Shape`.

- [ ] **Step 3: Add the variant** — in the enum after `Z52`: `Utn57Shape,` with a doc comment; `code_series` → `CodeSeries::Letter`; `canonical_str` → `"utn57_shape"`; `from_str` → `"utn57_shape" | "utn57shape" => Ok(CodeType::Utn57Shape),`. Update the enum doc: it is not a Java code type; it is the written-unit spelling of `Utn57`.

- [ ] **Step 4: Run** — `cargo test -q -p meco-core` — expected: passes; any exhaustive `match` elsewhere fails to compile and is fixed in the same step (the router's `translate_from`/`translate_to` only special-case, so nothing else should break).

### Task 2: The codec

**Files:**
- Create: `crates/meco-core/src/utn57_shape.rs`
- Modify: `crates/meco-core/src/lib.rs` (add `mod utn57_shape;`)

- [ ] **Step 1: Write the failing tests** (the `tests` module of the new file; the file must exist for them to compile, so create it with only the tests and empty functions first)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const SAIN: &str = "\u{1830}\u{1820}\u{1822}\u{180D}\u{1822}\u{180D}\u{1820}\u{180C}"; // ᠰᠠᠢ᠍ᠢ᠍ᠠ᠌, the canonical spelling
    const MONGOL_UN: &str =
        "\u{182E}\u{1823}\u{1820}\u{182D}\u{180C}\u{1828}\u{180B}\u{1828}\u{180B}\u{1823}\u{182F}\u{180E}\u{1824}\u{180B}\u{1820}\u{180C}";

    #[test]
    fn a_word_is_spelled_as_its_written_units() {
        assert_eq!(encode(SAIN).unwrap(), "SAIIA");
        assert_eq!(encode(MONGOL_UN).unwrap(), "MOAGNNOLMvsOA");
    }

    #[test]
    fn compact_and_joined_input_reach_the_same_canonical_text() {
        assert_eq!(decode("SAIIA").unwrap(), SAIN);
        assert_eq!(decode("S+A+I+I+A").unwrap(), SAIN);
        assert_eq!(decode("MOAGNNOLMvsOA").unwrap(), MONGOL_UN);
    }

    #[test]
    fn text_outside_a_word_passes_through_both_ways() {
        assert_eq!(encode(&format!("{SAIN} {SAIN}\u{1802} 2024 hello")).unwrap(), "SAIIA SAIIA\u{1802} 2024 hello");
        assert_eq!(decode("SAIIA SAIIA\u{1802} 2024 hello").unwrap(), format!("{SAIN} {SAIN}\u{1802} 2024 hello"));
        assert_eq!(decode("").unwrap(), "");
        assert_eq!(encode("  ").unwrap(), "  ");
    }

    #[test]
    fn an_uppercase_led_run_that_is_not_a_spelling_is_an_error() {
        let error = decode("Hello").unwrap_err();
        assert!(error.to_string().starts_with("UTN #57 conversion failed: utn57_shape:"), "{error}");
        assert!(matches!(decode("SAIIA Hello"), Err(MecoError::Utn57(_))));
    }

    #[test]
    fn words_are_uppercase_led_ascii_runs() {
        assert_eq!(split("SAIIA, 2024 hello S+A"), vec![Piece::Word("SAIIA"), Piece::Other(", 2024 hello "), Piece::Word("S+A")]);
    }
}
```

- [ ] **Step 2: Run** — `cargo test -q -p meco-core --lib utn57_shape` — expected: compile errors (`encode`, `decode`, `split`, `Piece` missing).

- [ ] **Step 3: Implement**

```rust
//! `utn57_shape`: UTN #57 text spelled as written units — ᠰᠠᠢᠨ as `SAIIA`.
//!
//! The encoding is `utn57` seen through mongol-norm's shape function. Writing renders each
//! Mongolian word of a UTN #57 text into its written units and concatenates their PascalCase
//! names; reading parses each spelled word (compact `SAIIA` or `+`-joined `S+A+I+I+A`) and asks
//! mongol-norm for the canonical UTN #57 spelling of that shape. Everything that is not a word
//! passes through unchanged, as in every other encoding.
//!
//! Design: `docs/superpowers/specs/2026-09-13-utn57-shape-encoding-design.md`.

use crate::error::MecoError;
use std::sync::OnceLock;
use zvvnmod_utn57::mongol_norm::{is_mongolian_word_char, Locale, Shaper};

/// The Hudum shaper, built once for the process from mongol-norm's static tables.
fn shaper() -> &'static Shaper {
    static SHAPER: OnceLock<Shaper> = OnceLock::new();
    SHAPER.get_or_init(|| Shaper::new(Locale::Mng))
}

fn error(reason: impl std::fmt::Display) -> MecoError {
    MecoError::Utn57(format!("utn57_shape: {reason}"))
}

/// UTN #57 text → shape text.
pub(crate) fn encode(utn57: &str) -> Result<String, MecoError> {
    let mut out = String::with_capacity(utn57.len());
    let mut word = String::new();
    let flush = |word: &mut String, out: &mut String| -> Result<(), MecoError> {
        if word.is_empty() {
            return Ok(());
        }
        for unit in shaper().shape(word).map_err(error)? {
            out.push_str(unit.as_str());
        }
        word.clear();
        Ok(())
    };
    for c in utn57.chars() {
        if is_mongolian_word_char(c) {
            word.push(c);
        } else {
            flush(&mut word, &mut out)?;
            out.push(c);
        }
    }
    flush(&mut word, &mut out)?;
    Ok(out)
}

/// Shape text → UTN #57 text.
pub(crate) fn decode(shape: &str) -> Result<String, MecoError> {
    let mut out = String::with_capacity(shape.len() * 3);
    for piece in split(shape) {
        match piece {
            Piece::Word(word) => {
                let shaper = shaper();
                let units = shaper.parse_written_units(word).map_err(error)?;
                out.push_str(&shaper.normalize_written_units(&units).map_err(error)?);
            }
            Piece::Other(text) => out.push_str(text),
        }
    }
    Ok(out)
}

/// One piece of shape text: a spelled word, or anything else.
#[derive(Debug, PartialEq, Eq)]
enum Piece<'a> {
    Word(&'a str),
    Other(&'a str),
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+'
}

/// Split shape text into spelled words and passthrough. A word is a maximal run of ASCII
/// letters, digits and `+` that begins with an uppercase letter — every unit name does, so a
/// run that begins otherwise (`2024`, `hello`) is not a spelling and passes through.
fn split(text: &str) -> Vec<Piece<'_>> {
    let mut pieces = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        let run_end = rest.find(|c: char| !is_word_char(c)).unwrap_or(rest.len());
        if run_end > 0 && rest.starts_with(|c: char| c.is_ascii_uppercase()) {
            pieces.push(Piece::Word(&rest[..run_end]));
            rest = &rest[run_end..];
            continue;
        }
        // Passthrough: this run (if any) and everything up to the next uppercase-led run.
        let mut end = run_end;
        loop {
            let after = &rest[end..];
            let next_run = after.find(is_word_char).unwrap_or(after.len());
            end += next_run;
            let run = &rest[end..];
            let len = run.find(|c: char| !is_word_char(c)).unwrap_or(run.len());
            if len == 0 || run.starts_with(|c: char| c.is_ascii_uppercase()) {
                break;
            }
            end += len;
        }
        if end == 0 {
            end = rest.len();
        }
        pieces.push(Piece::Other(&rest[..end]));
        rest = &rest[end..];
    }
    pieces
}
```

- [ ] **Step 4: Run** — `cargo test -q -p meco-core --lib utn57_shape` — expected: all pass. If `split` misbehaves on a boundary, fix `split`, not the test.

### Task 3: Route it

**Files:**
- Modify: `crates/meco-core/src/router.rs`
- Test: `crates/meco-core/tests/utn57_shape.rs`

- [ ] **Step 1: Write the failing integration tests**

```rust
//! `utn57_shape`: a UTN #57 word spelled as its written units, ᠰᠠᠢᠨ as `SAIIA`.
use meco_core::{translate, translate_with_warnings, CodeType};

const SAIN_DELEHI: &str = "\u{1830}\u{1820}\u{1822}\u{1828}";

#[test]
fn a_word_reaches_the_shape_encoding_from_any_source() {
    assert_eq!(translate(CodeType::Delehi, CodeType::Utn57Shape, SAIN_DELEHI).unwrap(), "SAIIA");
    assert_eq!(translate(CodeType::MenkLetter, CodeType::Utn57Shape, SAIN_DELEHI).unwrap(), "SAIIA");
}

#[test]
fn a_spelled_word_reaches_every_target_as_the_word_itself() {
    let utn57 = translate(CodeType::Delehi, CodeType::Utn57, SAIN_DELEHI).unwrap();
    assert_eq!(translate(CodeType::Utn57Shape, CodeType::Utn57, "SAIIA").unwrap(), utn57);
    assert_eq!(translate(CodeType::Utn57Shape, CodeType::Utn57, "S+A+I+I+A").unwrap(), utn57);
    assert_eq!(translate(CodeType::Utn57Shape, CodeType::Delehi, "SAIIA").unwrap(), SAIN_DELEHI);
    assert_eq!(
        translate(CodeType::Utn57Shape, CodeType::Zvvnmod, "SAIIA").unwrap(),
        translate(CodeType::Delehi, CodeType::Zvvnmod, SAIN_DELEHI).unwrap()
    );
}

#[test]
fn passthrough_and_the_detached_suffix_survive() {
    // ᠲᠠᠯ᠎ᠠ ᠶᠢᠨ, then a comma and a number.
    let delehi = "\u{1832}\u{1820}\u{182F}\u{180E}\u{1820}\u{202F}\u{1836}\u{1822}\u{1828}\u{1802} 2024";
    let shape = translate(CodeType::Delehi, CodeType::Utn57Shape, delehi).unwrap();
    assert_eq!(shape, "TALMvsAaMvsIIA\u{1802} 2024");
    assert_eq!(translate(CodeType::Utn57Shape, CodeType::Delehi, &shape).unwrap(), delehi);
}

#[test]
fn the_shape_encoding_is_its_own_identity_and_round_trips_through_the_hub() {
    assert_eq!(translate(CodeType::Utn57Shape, CodeType::Utn57Shape, "SAIIA").unwrap(), "SAIIA");
    let hub = translate(CodeType::Utn57Shape, CodeType::Zvvnmod, "MOAGNNOLMvsOA").unwrap();
    assert_eq!(translate(CodeType::Zvvnmod, CodeType::Utn57Shape, &hub).unwrap(), "MOAGNNOLMvsOA");
}

#[test]
fn output_carries_the_utn57_warnings_and_input_carries_none() {
    let padded = translate_with_warnings(CodeType::Zvvnmod, CodeType::Utn57Shape, "\u{E09C}").unwrap();
    assert_eq!(padded.warnings.len(), 1, "{:?}", padded.warnings);
    assert_eq!(padded.text, "ZwjGO");
    assert!(translate_with_warnings(CodeType::Utn57Shape, CodeType::Utn57, "SAIIA").unwrap().warnings.is_empty());
}

#[test]
fn a_spelling_that_is_not_one_is_an_error_not_passthrough() {
    let error = translate(CodeType::Utn57Shape, CodeType::Utn57, "Hello").unwrap_err();
    assert!(error.to_string().contains("utn57_shape:"), "{error}");
}
```

- [ ] **Step 2: Run** — `cargo test -q -p meco-core --test utn57_shape` — expected: failures (`MissTranslateRule` / `Unsupported`, since the router does not know the type).

- [ ] **Step 3: Wire the router**

In `translate_from`, before the `code_series` match:

```rust
    if ct == CodeType::Utn57Shape {
        // The written-unit spelling of a UTN #57 text: read it as that text.
        let utn57 = utn57_shape::decode(s)?;
        return translate_from(CodeType::Utn57, &utn57);
    }
```

In `translate_to`, before the `Utn57` branch:

```rust
    if ct == CodeType::Utn57Shape {
        // The UTN #57 conversion, then its shape; the warnings are that conversion's.
        let utn57 = translate_to(CodeType::Utn57, s)?;
        return Ok(Translation {
            text: utn57_shape::encode(&utn57.text)?,
            warnings: utn57.warnings,
        });
    }
```

Add `use crate::utn57_shape;` at the top and `mod utn57_shape;` in `lib.rs`.

- [ ] **Step 4: Run** — `cargo test -q -p meco-core` — expected: all pass (including the golden parity, which does not involve the new type).

### Task 4: Corpus property

**Files:**
- Modify: `crates/meco-core/tests/utn57_shape.rs`

- [ ] **Step 1: Add the test**

```rust
/// The shape is a pure function of the spelling, and normalizing a shape is canonical: wherever
/// hub → utn57 → hub is the identity, so is hub → utn57_shape → hub.
#[test]
fn the_shape_round_trip_is_the_utn57_round_trip() {
    let corpus = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden/corpus_delehi.txt"),
    )
    .expect("corpus should be readable");
    let (mut agree, mut checked) = (0usize, 0usize);
    for word in corpus.split_whitespace().take(400) {
        let Ok(hub) = translate(CodeType::Delehi, CodeType::Zvvnmod, word) else { continue };
        let Ok(utn57) = translate(CodeType::Zvvnmod, CodeType::Utn57, &hub) else { continue };
        let Ok(via_utn57) = translate(CodeType::Utn57, CodeType::Zvvnmod, &utn57) else { continue };
        let shape = translate(CodeType::Zvvnmod, CodeType::Utn57Shape, &hub).unwrap();
        let via_shape = translate(CodeType::Utn57Shape, CodeType::Zvvnmod, &shape).unwrap();
        checked += 1;
        if via_shape == via_utn57 {
            agree += 1;
        }
    }
    assert!(checked > 300, "corpus too small: {checked}");
    assert_eq!(agree, checked, "shape and utn57 round trips disagree");
}
```

- [ ] **Step 2: Run** — `cargo test -q -p meco-core --test utn57_shape` — expected: pass. If a word disagrees, print it and inspect: the likely cause is a shape mongol-norm cannot normalize back (`NoCanonicalEncoding`), which must then be documented in the spec's error section rather than papered over.

### Task 5: CLI and surfaces

**Files:**
- Modify: `crates/meco-core/src/bin/meco.rs` (help text: add `utn57_shape` after `utn57`)
- Modify: `crates/meco-core/tests/cli.rs`

- [ ] **Step 1: CLI test**

```rust
#[test]
fn converts_the_written_unit_spelling_in_both_directions() {
    let out = meco().args(["translate", "--from", "delehi", "--to", "utn57_shape", "\u{1830}\u{1820}\u{1822}\u{1828}"]).output().unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(out.stdout, b"SAIIA");
    let back = meco().args(["translate", "--from", "utn57_shape", "--to", "delehi", "SAIIA"]).output().unwrap();
    assert!(back.status.success(), "stderr: {}", String::from_utf8_lossy(&back.stderr));
    assert_eq!(back.stdout, "\u{1830}\u{1820}\u{1822}\u{1828}".as_bytes());
}
```

- [ ] **Step 2: Run** — `cargo test -q -p meco-core --test cli` — expected: pass already (names parse); add the help line and check `meco --help` lists `utn57_shape`.
- [ ] **Step 3: wasm** — `cargo check -q -p meco-wasm --target wasm32-unknown-unknown` — expected: clean.

### Task 6: Web page and docs

**Files:**
- Modify: `crates/meco-wasm/web/index.html` — `SIDE_TYPES` gains `"utn57_shape"` after `"utn57"`; `FONTS` gains `utn57_shape:"ui-monospace, Menlo, monospace"`; `STD_NOTE` gains a note: "UTN #57 text spelled as its written units (mongol-norm shape), ᠰᠠᠢᠨ = SAIIA / UTN #57 文本的书写单元拼写（mongol-norm shape），ᠰᠠᠢᠨ = SAIIA".
- Modify: `README.md`, `README.zh-CN.md` (encodings table row + a short section with the two examples), `USAGE.md` (names list), `crates/meco-wasm/README.md` (names list).

- [ ] **Step 1: Apply the edits; syntax-check the page script with `node -e` as before.**
- [ ] **Step 2: Full gates** — `cargo test -q --workspace`; `cargo check -q -p meco-wasm --target wasm32-unknown-unknown`; `rustfmt --check` on the new/changed Rust files only (never `cargo fmt --all` here).
- [ ] **Step 3: Commit** — `feat: add utn57_shape, the written-unit spelling of UTN #57` with the spec and plan.
