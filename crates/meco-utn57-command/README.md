# meco-utn57-command

Opt-in server/desktop adapter that adds canonical UTN #57 Unicode output to `meco` without
putting process spawning, Python, or runtime provisioning in `meco-core`.

## Conversion path

```text
declared meco source
→ meco-core
→ complete ZVVNMOD text
→ zvvnmod-utn57 0.1.0-alpha.3
→ positioned UTN #57 written-unit runs
→ external mongol-norm 0.0.4 command
→ canonical Unicode with passthrough spans restored
```

`meco-core` remains dependency-free, pure compute, and suitable for WASM, UniFFI, and C ABI
bindings. This adapter supports UTN #57 as a **target**. UTN #57 → legacy encoding conversion is
not implemented. All routes that do not involve `CodeType::Utn57` delegate directly to
`meco_core::translate`.

## Setup

Building this crate does not install Python or download `mongol-norm`. A server/desktop deployment
that converts formal ZVVNMOD shapes must explicitly install the reviewed backend once:

```bash
cargo install zvvnmod-utn57 --version 0.1.0-alpha.3 --locked
zvvnmod-install-mongol-norm
```

Passthrough-only and identity conversions do not start the backend.

## Rust API

```rust
use meco_core::CodeType;
use meco_utn57_command::translate;

let unicode = translate(CodeType::MenkShape, CodeType::Utn57, input)?;
```

The adapter routes every supported source through `CodeType::Zvvnmod`, then calls the
backend-neutral `zvvnmod_utn57::convert_zvvnmod_to_utn57` facade. It does not duplicate the
ZVVNMOD inventory, mapping tables, complete-text classification, or bridge protocol.

The published three-class ZVVNMOD complete-text contract is inherited unchanged:

- formal 139-code shapes, including ZVVNMOD Nirugu `U+E0E5`, are converted;
- legacy `U+E140..=U+E144` controls are discarded;
- all other text, including raw `U+180A`, `U+180E`, `U+202F`, and input `U+200D`, passes through
  unchanged and delimits adjacent shape runs.

Suffix-specific semantics for `U+202F` remain deferred; this adapter adds no suffix inference,
mapping relation, or inventory entry.
