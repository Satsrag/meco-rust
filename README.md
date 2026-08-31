# meco — Mongolian Encoding Converter (Rust)

A pure-Rust port of the Mongolian Encoding Converter, with thin bindings for **every platform**.
The core is verified **byte-exact** against the original Java library on an 11,492-row golden corpus
(all five encodings, every direction), so behaviour can't drift across languages.

蒙古文编码转换器的 Rust 版：一处核心，处处复用，且与原 Java 库逐字节对齐。

## Encodings

`Zvvnmod` (intermediate hub) · `Delehi` · `Menk_Shape` · `Menk_Letter` · `Z52` (zcode).
Portable conversions route through the Zvvnmod hub. `Oyun` remains recognized but unsupported.
`Utn57` is available as an output when `meco-core`'s opt-in `utn57-command` feature is enabled;
reverse conversion from UTN #57 remains unsupported.

## One core, every platform

```
                         meco-core  (pure by default, #![forbid(unsafe_code)])
                         translate(from, to, &str) -> Result<String, MecoError>
   ┌───────────────┬───────────────────┬────────────────────┬─────────────────────┐
 meco-wasm       meco-uniffi         meco-uniffi          meco-cabi             meco-cabi
 (wasm-bindgen)  (→ Swift)           (→ Kotlin)           (C ABI)               (C ABI)
   web/Node       iOS                 Android              PHP-FFI / cgo         JNI · Panama
```

The explicit `utn57-command` feature composes
`source → meco-core → ZVVNMOD → zvvnmod-utn57 → canonical Unicode` for server/desktop
deployments. Enabling the feature does not install Python packages or run a downloader; install the
reviewed backend once with:

```sh
cargo install zvvnmod-utn57 --version 0.1.0-alpha.3 --locked
zvvnmod-install-mongol-norm
```

The call remains the normal `meco-core` API:

```rust
use meco_core::{translate, CodeType};

let output = translate(CodeType::MenkShape, CodeType::Utn57, input)?;
```

Without the feature, a non-identity conversion targeting `CodeType::Utn57` returns
`MecoError::Unsupported(CodeType::Utn57)`. With the feature enabled, an unavailable or failing
backend returns `MecoError::Utn57`; existing bindings continue mapping errors to their normal
NULL/exception mechanism.

## Quick start

| Platform | Add it | Call |
|---|---|---|
| Rust | `meco-core = { path = "crates/meco-core" }` | `meco_core::translate(from, to, s)?` |
| Rust + UTN #57 output | `meco-core = { path = "crates/meco-core", features = ["utn57-command"] }` | `meco_core::translate(from, CodeType::Utn57, s)?` |
| PHP | `composer require zvvnmod/meco` | `Meco\Meco::translate(Meco::Z52, Meco::MENK_SHAPE, $s)` |
| Web/Node | `npm install meco-wasm` | `translate("z52", "menk_shape", s)` |
| iOS | SwiftPM / `pod 'Meco'` | `try translate(from: "z52", to: "menk_shape", input: s)` |
| Android | `implementation("com.zvvnmod:meco-android:…")` | `translate("z52", "menk_shape", s)` |

**[USAGE.md](USAGE.md)** — download the prebuilt artifacts from [Releases](../../releases) and use
them on each platform (C / Go / Python / Dart / Java / Android / Swift / ObjC / Web / PHP), no
package-manager account needed. **[DISTRIBUTION.md](DISTRIBUTION.md)** — optional registry publishing.

## Build & verify

```sh
cargo test --workspace     # unit tests + 11,492-row golden parity vs the Java oracle
```

The golden corpus (`crates/meco-core/tests/golden/golden.tsv`) is produced by `tools/oracle-java`
running the real Java `TranslateService`; the lookup tables under
`crates/meco-core/src/tables/generated/` are dumped from the live Java maps by `tools/table-gen`.

## Layout

- `crates/meco-core` — the engine (shape + letter subsystems, hub routing, generated tables)
- `crates/meco-cabi` · `crates/meco-uniffi` · `crates/meco-wasm` — bindings
- `bindings/php` · `bindings/swift` · `bindings/android` — per-ecosystem packages
- `tools/oracle-java` · `tools/table-gen` — the Java oracle + table generator
- `.github/workflows/release.yml` — builds & publishes every artifact on a `vX.Y.Z` tag

## License

Apache-2.0 (a port of the Java [east-mod/meco](https://github.com/east-mod/meco)).
