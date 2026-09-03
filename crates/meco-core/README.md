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
cargo install meco-core --version 0.3.1 --locked
meco translate --from z52 --to menk_shape 'text'
```

Omit the final text argument to read UTF-8 from stdin. Output is written unchanged to stdout without
an extra newline, so the command is safe in pipelines:

```sh
printf '%s' 'text' | meco translate --from z52 --to menk_shape
```

Run `meco --help` for the canonical encoding names and `meco --version` to verify the installed
release.

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
