# meco

[English](README.md) | [简体中文](README.zh-CN.md)

`meco` converts Mongolian text between ZVVNMOD, Delehi, MenkShape, MenkLetter, and Z52. The conversion core is written in Rust and verified byte-for-byte against the original Java implementation on an 11,492-row test corpus.

The `meco-core` crate provides both:

- a Rust library API;
- the `meco` command for desktop and server use.

Canonical UTN #57 Unicode output is available as an optional desktop/server feature. The Web, mobile, and prebuilt C packages remain pure Rust and do not start external commands.

## Supported encodings

| CLI name | Description | Portable source | Portable target |
|---|---|---:|---:|
| `zvvnmod` | Internal shape-oriented interchange format used by meco | Yes | Yes |
| `delehi` | Delehi Unicode letter convention | Yes | Yes |
| `menk_shape` | Menk positional shape encoding | Yes | Yes |
| `menk_letter` | Menk letter convention | Yes | Yes |
| `z52` | Z52/zcode positional encoding | Yes | Yes |
| `utn57` | Unicode output following the reviewed UTN #57 mapping | No | Optional |
| `oyun` | Reserved by the original API | No | No |

MenkLetter and Delehi use many of the same Unicode code points, but they apply different contextual rules. `meco` does not guess the source encoding. Choose `--from` from the application, input method, font system, or database column that produced the text.

## Install the command

### Requirements

- Rust 1.82 or newer;
- Cargo on `PATH`.

Install Rust with [rustup](https://rustup.rs/) if `cargo --version` is unavailable.

### Standard CLI

Install the published `meco-core 0.2.1` crate:

```sh
cargo install meco-core --version 0.2.1 --locked
```

Check the installation:

```sh
meco --version
meco --help
```

Expected version:

```text
meco 0.2.1
```

### Convert text from an argument

```sh
meco translate --from z52 --to menk_shape 'text'
```

Use the canonical encoding names shown in the table above. The compatibility aliases `menkshape` and `menkletter` are also accepted.

### Read from stdin

Omit the final text argument to read UTF-8 from stdin:

```sh
printf '%s' 'text' | meco translate --from z52 --to menk_shape
```

This mode is suitable for files, shell pipelines, and server jobs:

```sh
meco translate --from z52 --to delehi < input.txt > output.txt
```

`meco` writes only the converted UTF-8 bytes to stdout. It does not append a newline. Errors go to stderr and return a non-zero exit status.

For interactive use, add a newline after the command:

```sh
meco translate --from z52 --to delehi 'text'; echo
```

On zsh, a `%` displayed immediately after the result is the shell's end-of-line marker. It is not part of the converted text.

## Install UTN #57 output

UTN #57 output uses a reviewed ZVVNMOD-to-positioned-written-unit mapping and the pinned `mongol-norm 0.0.4` normalization backend. This path is intended for desktop and server deployments.

### 1. Install `meco` with the feature enabled

```sh
cargo install meco-core \
  --version 0.2.1 \
  --features utn57-command \
  --locked
```

You can run this command over an existing standard installation. Cargo will replace the `meco` executable with the feature-enabled build.

### 2. Install the backend helper

```sh
cargo install zvvnmod-utn57 \
  --version 0.1.0-alpha.3 \
  --locked
```

### 3. Install the pinned Python backend

```sh
zvvnmod-install-mongol-norm
```

The installer creates a user-local, hash-locked installation of `mongol-norm 0.0.4`. It does not require root access and does not modify the system Python environment.

### 4. Convert to UTN #57

```sh
meco translate --from z52 --to utn57 'ᡳᡬᡦ ᢌᡭᡪᢊᡱᡱᡭᢐ ᢋᡭᡬᢎᡭᡧ'; echo
```

All supported legacy sources can target UTN #57:

```sh
meco translate --from menk_letter --to utn57 '...'
meco translate --from delehi --to utn57 '...'
meco translate --from menk_shape --to utn57 '...'
meco translate --from zvvnmod --to utn57 '...'
```

Reverse conversion from UTN #57 is not implemented:

```text
--from utn57 → unsupported
```

### UTN #57 troubleshooting

#### `conversion not supported for Utn57`

The installed `meco` was built without the optional feature. Reinstall it with:

```sh
cargo install meco-core --version 0.2.1 --features utn57-command --locked
```

#### `FileNotFoundError: .../.local/share/zvvnmod-utn57`

The Rust feature is present, but the normalization backend has not been installed. Run:

```sh
cargo install zvvnmod-utn57 --version 0.1.0-alpha.3 --locked
zvvnmod-install-mongol-norm
```

#### The output contains FVS, MVS, or ZWJ

That is expected. UTN #57 serialization uses standard Unicode Mongolian letters and format controls to request specific written forms. Inspect code points rather than relying on one font's rendering.

#### MenkLetter and Delehi produce different results

They are different source conventions even though both use Unicode Mongolian letters. Check where the source text came from. Do not switch the `--from` value based only on how the text looks.

## Use the Rust library

Add the default, pure Rust library:

```sh
cargo add meco-core@0.2.1
```

Or add it to `Cargo.toml`:

```toml
[dependencies]
meco-core = "0.2.1"
```

Convert text:

```rust
use meco_core::{translate, CodeType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "text";
    let output = translate(CodeType::Z52, CodeType::MenkShape, input)?;
    print!("{output}");
    Ok(())
}
```

The default library has no external runtime or command dependency.

### Rust library with UTN #57 output

```toml
[dependencies]
meco-core = { version = "0.2.1", features = ["utn57-command"] }
```

```rust
use meco_core::{translate, CodeType};

let output = translate(CodeType::MenkLetter, CodeType::Utn57, input)?;
```

The feature-enabled library uses the same external backend as the CLI. Install it once with `zvvnmod-install-mongol-norm` on every machine that performs UTN #57 conversion.

## Prebuilt release packages

Download packages from the [v0.2.1 release](https://github.com/Satsrag/meco-rust/releases/tag/v0.2.1).

| Platform | Release asset |
|---|---|
| Linux x86_64 C ABI | `meco-c-linux-x86_64.zip` |
| Linux AArch64 C ABI | `meco-c-linux-aarch64.zip` |
| macOS Apple Silicon C ABI | `meco-c-macos-arm64.zip` |
| macOS Intel C ABI | `meco-c-macos-x86_64.zip` |
| Windows x86_64 C ABI | `meco-c-windows-x86_64.zip` |
| iOS Swift | `MecoSwift.xcframework.zip` |
| Apple C ABI | `MecoC.xcframework.zip` |
| Android | `meco-android-release.aar` |
| Browser/WebAssembly | `meco-wasm-web-0.2.1.tgz` |
| Node.js/WebAssembly | `meco-wasm-nodejs-0.2.1.tgz` |

The C archives include the header and static/dynamic libraries for the target. Go, Python, PHP, Java, Dart, and other runtimes can load the C ABI. Swift, Android, browser, and Node.js have dedicated packages.

See [USAGE.md](USAGE.md) for C, C++, Go, Python, Dart, Java, Android, Swift, Objective-C, browser, Node.js, and PHP examples.

The prebuilt packages do not include the command-backed UTN #57 feature. Portable conversions among ZVVNMOD, Delehi, MenkShape, MenkLetter, and Z52 work in those packages.

## Conversion model

Portable conversions use ZVVNMOD as the hub:

```text
source encoding
→ source-specific letter or shape decoder
→ ZVVNMOD
→ target-specific letter or shape encoder
→ target text
```

UTN #57 output adds two reviewed stages:

```text
source encoding
→ meco-core
→ ZVVNMOD positioned shapes
→ zvvnmod-utn57 positioned written units
→ mongol-norm 0.0.4
→ Unicode letters and format controls
```

MenkLetter and Delehi are letter-level source conventions. MenkShape and Z52 are shape-oriented sources. A shape-oriented source does not always retain enough information to recover one unique phonetic spelling. UTN #57 output from those sources is a reviewed, shape-preserving Unicode serialization, not a dictionary or spelling reconstruction.

## Data safety and round trips

Keep the original text when migrating a corpus. Conversions can normalize FVS/MVS sequences, collapse several legacy spellings into one target spelling, or lose source-specific boundary information. Current UTN #57 conversion is output-only.

A practical storage model is:

```text
raw_source      original text and its declared encoding
normalized      converted Unicode/UTN #57 derivative
search_text     transliteration or another search-oriented representation
```

Do not detect MenkLetter versus Delehi from code point ranges alone. Store the source encoding with the text.

## Build and test

Clone the repository and run:

```sh
git clone https://github.com/Satsrag/meco-rust.git
cd meco-rust
cargo test --workspace --locked
```

Build the CLI:

```sh
cargo build -p meco-core --bin meco --release --locked
```

Build the feature-enabled CLI:

```sh
cargo build -p meco-core --bin meco --release --features utn57-command --locked
```

The portable conversion matrix is checked against the original Java meco implementation on 11,492 golden rows.

## Repository layout

```text
crates/meco-core      Rust library and meco CLI
crates/meco-cabi      C ABI
crates/meco-uniffi    Swift/Kotlin bindings
crates/meco-wasm      browser and Node.js WebAssembly
bindings/             platform packaging
.github/workflows/    CI and release automation
```

## Documentation

- [中文 README](README.zh-CN.md)
- [Platform examples](USAGE.md)
- [Distribution and release process](DISTRIBUTION.md)
- [`meco-core` on crates.io](https://crates.io/crates/meco-core)
- [`meco-core` API documentation](https://docs.rs/meco-core)
- [GitHub releases](https://github.com/Satsrag/meco-rust/releases)

## License

Apache-2.0. This project is a Rust port of the Java [east-mod/meco](https://github.com/east-mod/meco) implementation.
