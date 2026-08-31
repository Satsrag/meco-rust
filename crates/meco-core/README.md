# meco-core

`meco-core` is the pure-by-default Rust engine for the Mongolian Encoding Converter. It supports
`Zvvnmod`, `Delehi`, `MenkShape`, `MenkLetter`, and `Z52`; conversions route through the Zvvnmod
hub. The minimum supported Rust version is 1.82.

## Basic use

```rust
use meco_core::{translate, CodeType};

let output = translate(CodeType::MenkShape, CodeType::Zvvnmod, input)?;
```

## Optional UTN #57 output

Canonical UTN #57 Unicode output is available through the same API with the explicit
`utn57-command` feature:

```toml
[dependencies]
meco-core = { version = "0.1.0", features = ["utn57-command"] }
```

```rust
use meco_core::{translate, CodeType};

let output = translate(CodeType::MenkShape, CodeType::Utn57, input)?;
```

The feature adds the command-backed `zvvnmod-utn57` integration but does not install Python or
run a downloader during the Cargo build. A server or desktop deployment that converts formal
ZVVNMOD shapes must install the reviewed backend explicitly:

```sh
cargo install zvvnmod-utn57 --version 0.1.0-alpha.3 --locked
zvvnmod-install-mongol-norm
```

Without the feature, a non-identity conversion targeting `CodeType::Utn57` returns
`MecoError::Unsupported(CodeType::Utn57)`. With the feature enabled, an unavailable or failing
backend returns `MecoError::Utn57(reason)`. Reverse conversion from UTN #57 remains unsupported.
Identity and blank-input conversions retain the normal short-circuit behavior and do not start the
backend.

The optional conversion path is:

```text
source encoding
→ meco-core ZVVNMOD hub
→ zvvnmod-utn57 0.1.0-alpha.3
→ explicitly installed mongol-norm command
→ canonical Unicode
```

For bindings, distribution, and the Java-oracle verification details, see the
[meco-rust repository](https://github.com/Satsrag/meco-rust).
