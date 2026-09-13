# `utn57_shape`: the written-unit spelling of UTN #57 text

Date: 2026-09-13. Status: approved.

## Purpose

Add an encoding whose text is the *shape* of a UTN #57 word — the sequence of written units
mongol-norm's `shape` renders it into — so a word can be entered and read as `SAIIA` rather than as
letters and FVS marks. It reads and writes, like every other encoding, and it never disagrees with
the `utn57` encoding: it is `utn57` seen through mongol-norm's shape function.

## Name

`CodeType::Utn57Shape`, canonical string `utn57_shape`, alias `utn57shape` — the `menk_shape`
pattern. Its `CodeSeries` is `Letter`, which only matters to the legacy dispatch that never sees it.

## Text format

- A Mongolian word is spelled as the PascalCase names of its written units, concatenated:
  ᠰᠠᠢᠨ → `SAIIA`. The structural units are tokens inside the word: `Mvs`, `Nirugu`, `Zwj` —
  ᠮᠣᠩᠭᠣᠯ᠎ᠤᠨ → `MOAGNNOLMvsOA`, ᠲᠠᠯ᠎ᠠ ᠶᠢᠨ → `TALMvsAa MvsIIA`. The names are mongol-norm's
  (`WrittenUnit::as_str`), so `K2`, `Aa`, `Sh`, `Dd`, … spell as mongol-norm spells them.
- Output is always compact. Input also accepts the explicit `+`-joined form (`S+A+I+I+A`), because
  that is what mongol-norm's `shape` command prints.
- Everything that is not a Mongolian word passes through unchanged: spaces, punctuation, digits,
  other scripts. Words are separated by the ordinary space between them, as in the other encodings.

## Reading (`utn57_shape` → hub)

1. Split the text into words and passthrough. A word is a maximal run of `[A-Za-z0-9+]` that begins
   with an ASCII uppercase letter; every unit name begins with one (`K2` and `B2` carry a digit
   inside; a `+`-joined word begins with its first unit). A run that does not begin with an uppercase
   letter (`2024`, `hello`) is passthrough.
2. Each word goes through `Shaper::parse_written_units` (compact or `+`-joined) and then
   `Shaper::normalize_written_units`, which yields the canonical UTN #57 spelling of that shape.
3. The assembled UTN #57 text takes the existing `utn57` → hub path (`zvvnmod-utn57`'s reverse
   direction), so the hub is exactly the hub the `utn57` encoding would have reached.

A word that begins with an uppercase letter but does not parse — an unknown unit, an ambiguous
compact segmentation, a shape the normalize table does not cover — is an error, not passthrough:
the encoding's alphabet is ASCII, so silently keeping the text would hand a wrong word downstream.
The error is `MecoError::Utn57` with the message prefixed `utn57_shape:` and mongol-norm's own
reason, which names the offending index or unit. No new error variant, so the C ABI and UniFFI
bindings are untouched.

## Writing (hub → `utn57_shape`)

1. The hub takes the existing hub → `utn57` path, warnings included.
2. The UTN #57 text is split into Mongolian words (`mongol_norm::is_mongolian_word_char`, the
   split `convert_utn57_to_zvvnmod` already uses) and passthrough.
3. Each word goes through `Shaper::shape`; the unit names are concatenated. Passthrough is copied.

The warnings of the conversion are the `utn57` conversion's warnings — an invented ZWJ or a
collapsed suffix boundary is a property of the spelling the shape was rendered from.

## Where it lives

- `crates/meco-core/src/utn57_shape.rs`: the codec — `decode(text) -> Result<String, MecoError>`
  (shape text → UTN #57 text) and `encode(utn57: &str) -> Result<String, MecoError>` (UTN #57 text
  → shape text), plus the word splitter. One `mongol_norm::Shaper` for the process in a `OnceLock`,
  built from `Locale::Mng`; `mongol_norm` is reached through `zvvnmod_utn57::mongol_norm`, so no new
  dependency.
- `router.rs`: `translate_from(Utn57Shape)` decodes and continues as `Utn57`;
  `translate_to(Utn57Shape)` converts as `Utn57` and encodes the text, keeping the warnings.
- `code_type.rs`: the variant, its strings, its series.
- `bin/meco.rs`: the help text lists `utn57_shape`.
- The wasm, C and UniFFI bindings parse encoding names, so they gain the encoding without a code
  change; their READMEs list it.
- `crates/meco-wasm/web/index.html`: `utn57_shape` on both selectors, a system monospace font for
  that pane (the text is ASCII), and a selector note saying what it is. The wrong-encoding
  highlighter never flags ASCII, so it needs nothing.

## Testing

- Unit tests on the codec: compact and `+`-joined input reach the same text; a lowercase- or
  digit-led run passes through; an uppercase-led run that does not parse is an error naming the
  reason; structural tokens round-trip; empty and whitespace-only input.
- Integration tests through `translate`: delehi ᠰᠠᠢᠨ → `utn57_shape` is `SAIIA`; `SAIIA` →
  `utn57` equals delehi → `utn57` of ᠰᠠᠢᠨ; a multi-word text with a space, punctuation and a
  detached suffix keeps the passthrough and spells `Mvs`; `translate_with_warnings` carries the
  `utn57` warnings on output and none on input.
- Property over the Delehi corpus: hub → `utn57_shape` → hub is the identity wherever hub →
  `utn57` → hub already is (the shape is a pure function of the spelling, and normalization of a
  shape is canonical).
- CLI test for both directions; `cargo check -p meco-wasm --target wasm32-unknown-unknown`.

## Release

Adding a `CodeType` variant is an API change for callers that match on it exhaustively, so the
release that ships this is 0.5.0.
