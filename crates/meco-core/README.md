# meco-core

`meco-core` is the pure-Rust engine for the Mongolian Encoding Converter. It converts among
`Zvvnmod`, `Delehi`, `MenkShape`, `MenkLetter`, `Z52`, and canonical UTN #57 Unicode in both
directions; conversions route through the Zvvnmod hub. Everything runs in process with
no I/O, so the crate builds for `wasm32-unknown-unknown` as well as native targets. The minimum
supported Rust version is 1.82.

## Basic use

```rust
use meco_core::{translate, CodeType};

let input = "\u{E0E5}";
let output = translate(CodeType::MenkShape, CodeType::Zvvnmod, input)
    .expect("ZVVNMOD conversion should succeed");
```

## Command line

The package also installs a `meco` binary without changing how Rust projects depend on the library:

```sh
cargo install meco-core --version 0.5.1 --locked
meco translate --from z52 --to menk_shape 'text'
```

Omit the final text argument to read UTF-8 from stdin. Output is written unchanged to stdout without
an extra newline, so the command is safe in pipelines:

```sh
printf '%s' 'text' | meco translate --from z52 --to menk_shape
```

Run `meco --help` for the canonical encoding names and `meco --version` to verify the installed
release.

## Optional suffix separator repair

For MenkLetter or Delehi input with lost suffix separators, enable input repair before conversion:

```rust
use meco_core::{translate_with_options, CodeType, TranslationOptions};

let result = translate_with_options(
    CodeType::MenkLetter,
    CodeType::MenkLetter,
    "ᠤᠯᠤᠰ ᠤᠨ",
    &TranslationOptions { repair_suffix_separators: true },
).unwrap();
assert_eq!(result.text, "ᠤᠯᠤᠰ\u{202F}ᠤᠨ");
assert_eq!(result.warnings.len(), 1);
```

With the CLI built from this checkout, place the option after the target encoding:

```sh
meco translate --from menk_letter --to menk_shape --repair-suffix-separators 'ᠤᠯᠤᠰ ᠤᠨ'
```

The option also works with stdin. Repairs are listed on stderr; stdout contains only converted
text. Use `--` before a literal text argument that matches the option name.

Repair is **off by default**. It replaces a single space (U+0020) or NBSP (U+00A0) between a
Mongolian word and one of these exact suffix spellings with NNBSP (U+202F):

`ᠶᠢᠨ`, `ᠤᠨ`, `ᠦᠨ`, `ᠤ`, `ᠦ`, `ᠶᠢ`, `ᠢ`, `ᠳᠤ`, `ᠳᠦ`, `ᠲᠤ`, `ᠲᠦ`,
`ᠳᠤᠷ`, `ᠳᠦᠷ`, `ᠲᠤᠷ`, `ᠲᠦᠷ`, `ᠠᠴᠠ`, `ᠡᠴᠡ`, `ᠢᠶᠠᠷ`, `ᠢᠶᠡᠷ`.

Another 22 spellings use a matching masculine/feminine vowel check: iyan/iyen, luγ-a/lüge,
nuγud/nügüd, ud/üd, daγan/degen, taγan/tegen, yuγan/yügen, ačaγan/ečegen, duni/düni,
tuni/tüni and dahi/dehi. Only iyan/iyen additionally require a consonant-final segment.
The check skips mixed-harmony and neutral-only segments, and accepts a final consonant +
MVS + A/E while leaving other control-bearing contexts unchanged. Internal MVS characters
are preserved. In suffix chains the check uses the immediately preceding segment.
The original 19 rules do not perform this additional context check.

See the [particle mapping audit](../../docs/suffix-separator-repair.md) for the exact Unicode
spellings, evidence and decisions for all 49 entries in the pinned font table. Repair supports
41 spellings in total; this is not a complete Mongolian suffix inventory. Ordinal dugar/düger
requires numeral context and is excluded, along with discourse particles whose separator
convention cannot be inferred by this rule. A font's particle table is not a repair allowlist.

This is an explicit spelling heuristic, not grammatical validation: it cannot tell whether a
suffix-like token was intended as a separate word or a quoted letter. Common ambiguous forms
such as `ᠪᠠᠷ` (bar) and `ᠲᠠᠢ` (tai) are excluded. Concatenated words, FVS-bearing suffix
spellings, existing NNBSP/MVS, tabs, newlines and runs of multiple spaces are left alone.
The suffix inventory is based on the separated suffix examples in
[L2/19-130](https://unicode.org/L2/L2019/19130-mwg3-8-mong-spec-r.pdf);
the bar ambiguity is described in
[L2/18-293](https://www.unicode.org/L2/L2018/18293-nnbsp-solution.pdf).

Each change returns `Warning::RepairedSuffixSeparator` with the **original input UTF-8 byte
offset** and replaced character, followed by any conversion warnings. Repair runs even for
same-encoding conversions. Any supported target can be used; the usual conversion rules then
represent the repaired boundary in that target. Enabling repair for other source encodings
returns `MecoError::UnsupportedInputRepair`, since their suffix spellings require different rules.
The existing `translate` and `translate_with_warnings` APIs keep their behavior. This option is
currently exposed in Rust and the CLI; the existing platform bindings still use the default API.

## UTN #57 output

Canonical UTN #57 Unicode output is part of the default build and uses the same API:

```rust
use meco_core::{translate, CodeType};

let input = "\u{E0E5}";
let output = translate(CodeType::MenkShape, CodeType::Utn57, input)
    .expect("UTN #57 conversion should succeed");
assert_eq!(output, "\u{180A}");
```

The conversion is performed in process by the pure-Rust `zvvnmod-utn57` crate and its pinned
`mongol-norm` normalizer. No Python, subprocess, installer, filesystem, or network access is
involved, so the same code path runs on servers, desktops, mobile, and WebAssembly.

A failing conversion returns `MecoError::Utn57(reason)`. Reverse conversion from UTN #57 remains
unsupported and returns `MecoError::Unsupported(CodeType::Utn57)`. Identity and blank-input
conversions keep the normal short-circuit behavior.

The `utn57-command` feature from 0.2.x is kept as a deprecated no-op so existing
`--features utn57-command` commands still build; it no longer changes anything.

The conversion path is:

```text
source encoding
→ meco-core ZVVNMOD hub
→ zvvnmod-utn57 0.1.0 positioned written units
→ mongol-norm 0.1.1 (linked in)
→ canonical Unicode
```

For bindings, distribution, and the Java-oracle verification details, see the
[meco-rust repository](https://github.com/Satsrag/meco-rust).
