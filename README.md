# meco — Mongolian Encoding Converter (Rust)

A pure-Rust port of the Mongolian Encoding Converter, with thin bindings for **every platform**.
The core is verified **byte-exact** against the original Java library on an 11,492-row golden corpus
(all five encodings, every direction), so behaviour can't drift across languages.

蒙古文编码转换器的 Rust 版：一处核心，处处复用，且与原 Java 库逐字节对齐。

## Encodings

`Zvvnmod` (intermediate hub) · `Delehi` · `Menk_Shape` · `Menk_Letter` · `Z52` (zcode).
Conversions route through the Zvvnmod hub. (`Oyun` / `Utn57` are recognized but not yet supported.)

## One core, every platform

```
                         meco-core  (pure Rust, no deps, #![forbid(unsafe_code)])
                         translate(from, to, &str) -> Result<String, MecoError>
   ┌───────────────┬───────────────────┬────────────────────┬─────────────────────┐
 meco-wasm       meco-uniffi         meco-uniffi          meco-cabi             meco-cabi
 (wasm-bindgen)  (→ Swift)           (→ Kotlin)           (C ABI)               (C ABI)
   web/Node       iOS                 Android              PHP-FFI / cgo         JNI · Panama
```

## Quick start

| Platform | Add it | Call |
|---|---|---|
| Rust | `meco-core = { path = "crates/meco-core" }` | `meco_core::translate(from, to, s)?` |
| PHP | `composer require zvvnmod/meco` | `Meco\Meco::translate(Meco::Z52, Meco::MENK_SHAPE, $s)` |
| Web/Node | `npm install meco-wasm` | `translate("z52", "menk_shape", s)` |
| iOS | SwiftPM / `pod 'Meco'` | `try translate(from: "z52", to: "menk_shape", input: s)` |
| Android | `implementation("com.zvvnmod:meco-android:…")` | `translate("z52", "menk_shape", s)` |

Full install + publish steps: **[DISTRIBUTION.md](DISTRIBUTION.md)**.

## Build & verify

```sh
cargo test                 # 21 unit tests + 11,492-row golden parity vs the Java oracle
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
