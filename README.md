# meco — Mongolian Encoding Converter (Rust)

A pure-Rust port of the Mongolian Encoding Converter, with thin bindings for **every platform**.
The core is verified **byte-exact** against the original Java library on an 11,492-row golden corpus
(all five encodings, every direction), so behaviour can't drift across languages.

蒙古文编码转换器的 Rust 版：一处核心，处处复用，且与原 Java 库逐字节对齐。

## 安装与使用 / Install and use

### Desktop / server command line

`meco-core` 同时提供 Rust library 和 `meco` 命令。安装 `0.2.1`：

```sh
cargo install meco-core --version 0.2.1 --locked
meco --version
```

直接转换一段文本：

```sh
meco translate --from z52 --to menk_shape 'text'
```

省略最后的文本参数时，命令从 stdin 读取 UTF-8，适合 shell pipeline 和服务器任务：

```sh
printf '%s' 'text' | meco translate --from z52 --to menk_shape
```

转换结果原样写入 stdout，不额外添加换行；错误写入 stderr 并返回非零状态。运行
`meco --help` 查看帮助。可用编码名：

```text
zvvnmod  delehi  menk_shape  menk_letter  oyun  utn57  z52
```

`oyun` 目前不支持；`utn57` 仅作为输出目标，并需要下面的可选功能。

### UTN #57 command output

Desktop/server 上需要输出 UTN #57 时，安装带 `utn57-command` feature 的同一个命令，并执行
一次 backend 安装：

```sh
cargo install meco-core --version 0.2.1 --features utn57-command --locked
cargo install zvvnmod-utn57 --version 0.1.0-alpha.3 --locked
zvvnmod-install-mongol-norm

meco translate --from menk_shape --to utn57 'text'
```

默认安装保持 pure-by-default，不会下载或运行 Python backend。Web、Android、iOS 和预编译 C
产物不包含 command backend。

### Rust library

在其他 Rust 项目中，`meco-core` 仍然作为普通 library 使用；增加 CLI 不会改变 library API：

```sh
cargo add meco-core@0.2.1
```

```rust
use meco_core::{translate, CodeType};

let output = translate(CodeType::Z52, CodeType::MenkShape, input)?;
```

Rust library 需要 UTN #57 输出时：

```toml
[dependencies]
meco-core = { version = "0.2.1", features = ["utn57-command"] }
```

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

## Other platform packages

| Platform | Add it | Call |
|---|---|---|
| PHP | `composer require zvvnmod/meco` | `Meco\Meco::translate(Meco::Z52, Meco::MENK_SHAPE, $s)` |
| Browser/web | Install `meco-wasm-web-*.tgz` from the GitHub Release | `translate("z52", "menk_shape", s)` |
| Node.js | Install `meco-wasm-nodejs-*.tgz` from the GitHub Release | `translate("z52", "menk_shape", s)` |
| iOS | Download `MecoSwift.xcframework.zip` from the GitHub Release | `try translate(from: "z52", to: "menk_shape", input: s)` |
| Android | `implementation("com.zvvnmod:meco-android:…")` | `translate("z52", "menk_shape", s)` |

**[USAGE.md](USAGE.md)** — download the prebuilt artifacts from [Releases](../../releases) and use
them on each platform (C / Go / Python / Dart / Java / Android / Swift / ObjC / Web / PHP), no
package-manager account needed. **[DISTRIBUTION.md](DISTRIBUTION.md)** — optional registry publishing.

## Build & verify

`meco-core` declares Rust 1.82 as its minimum supported Rust version, matching the optional
`zvvnmod-utn57` dependency.

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
