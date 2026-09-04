# Distributing meco — easy install for every platform

One Rust core (`meco-core`), one thin binding per ecosystem, and one release pipeline
(`.github/workflows/release.yml`) that builds every native artifact from a single version tag.
**Recommendation:** publish `meco-rust/` as its own Git repository (so the workflow, `Package.swift`,
and the per-language packages sit at the repo root, like `meco_php`/`meco_dart` are separate repos).

## How a consumer adds it (the goal: one line)

| Platform | Add to a project | Backed by |
|---|---|---|
| **Desktop / server CLI** | `cargo install meco-core --version 0.4.1 --locked` | `meco-core` binary + library |
| **PHP** (server) | `composer require zvvnmod/meco` | `meco-cabi` C ABI via FFI |
| **Browser / web bundler** | Install `meco-wasm-web-*.tgz` from the GitHub Release | `meco-wasm` (wasm-bindgen web target) |
| **Node.js** | Install `meco-wasm-nodejs-*.tgz` from the GitHub Release | `meco-wasm` (wasm-bindgen nodejs target) |
| **iOS** (SwiftPM) | Download `MecoSwift.xcframework.zip` from the GitHub Release | `meco-uniffi` (Swift) |
| **iOS** (CocoaPods) | `pod 'Meco'` | `meco-uniffi` (Swift) |
| **Android** (Gradle) | `implementation("com.zvvnmod:meco-android:0.4.1")` | `meco-uniffi` (Kotlin) |
| **Go / Java / Python…** | load `libmeco.{so,dylib}` (cgo / Panama-JNI / ctypes) | `meco-cabi` C ABI |

Usage is the same everywhere: `translate(from, to, input)` with names
`zvvnmod` / `delehi` / `menk_shape` / `menk_letter` / `z52`.

## Verification status

| Binding | Verified here? |
|---|---|
| PHP FFI | ✅ 200/200 byte-exact vs Java (this machine, PHP 8.5) |
| WASM (Node) | ✅ 200/200 byte-exact |
| Swift (UniFFI) | ✅ 120/120 byte-exact (host arm64-apple-darwin) |
| C ABI | ✅ 200/200 byte-exact (C smoke) |
| Kotlin / iOS-device / Android | generated + recipes; built by the release CI (needs NDK / Xcode) |

## Publishing

A tag `vX.Y.Z` builds every downloadable platform artifact, publishes `meco-core` to crates.io,
then creates the GitHub Release. Every stage is fail-closed. The tagged version must equal the
`meco-core` Cargo version, and the tagged commit must already be on upstream `main`.
The workflow resolves the current remote tag again immediately before publication and before
creating the GitHub Release, so a moved tag fails closed.

The repository owner must configure Trusted Publishing under the `meco-core` crate's crates.io
settings with GitHub owner `Satsrag`, repository `meco-rust`, workflow filename `release.yml`, and
environment `crates-io`. The publish job grants only `id-token: write`; the SHA-pinned crates.io auth
action exchanges GitHub's OIDC identity for a 30-minute publishing token and exposes it only to the
`cargo publish` step. No long-lived `CARGO_REGISTRY_TOKEN` secret is stored in GitHub.

Keep the protected GitHub environment named `crates-io`, require release review, allow only `v*`
tag deployments, and maintain a repository tag ruleset restricting creation, updates, and deletion
of `v*` tags to release maintainers.

The workflow builds and uploads per-platform `libmeco`, the iOS `MecoSwift.xcframework` and
`MecoC.xcframework`, the Android `.aar`, and the wasm npm tarball. The GitHub Release is created
only after crates.io publication succeeds. A rerun after a partial failure is safe: if the exact
crate version already exists, the workflow proceeds only when its crates.io checksum matches the
locally rebuilt package archive.

This workflow does not publish the PHP wrapper to Packagist, the wasm package to npm, the Apple
bindings to CocoaPods/SwiftPM, or the Android binding to Maven. It provides their versioned GitHub
Release assets; those ecosystem registries remain separate follow-up steps.

### Rust → crates.io

Rust consumers get the UTN #57 target through the normal core API with no feature flag:

```toml
meco-core = "0.4.1"
```

The pure-Rust `zvvnmod-utn57` backend is a regular dependency, so `cargo publish` and downstream
builds need nothing beyond crates.io. `utn57-command` remains accepted as a deprecated no-op
feature.

### PHP → Packagist
1. Push the repo to GitHub; submit it once at https://packagist.org (auto-updates via webhook).
2. Native libs are attached to the GitHub Release. The current PHP wrapper still needs `MECO_LIB`
   pointed at the downloaded library, or a local build via `bindings/php/scripts/build-lib.sh`.

### Web → npm
The `wasm` CI job builds separate browser/web and Node.js packages, runs `npm pack` for each, then
attaches both `.tgz` files to the GitHub Release. It does not call `npm publish`; consumers install the
matching downloaded tarball directly.

### iOS → SwiftPM / CocoaPods
CI builds `MecoSwift.xcframework` and `MecoC.xcframework`. For SwiftPM, point the
binary targets in `Package.swift` at the release URLs and checksums (or keep them at local paths while
developing), and copy `sw/meco_uniffi.swift` from the release archive into `Sources/Meco/` before
publishing the split Swift package. For CocoaPods, verify the versioned release-asset URL in
`Meco.podspec`, then push it to your trunk/specs repo.

### Android → Maven Central or JitPack
CI builds the AAR (`cargo ndk` → `.so` per ABI + UniFFI Kotlin + `gradlew assembleRelease`).
Publish to Maven Central (`maven-publish` + signing) for `implementation("com.zvvnmod:meco-android:…")`,
or enable JitPack for `implementation("com.github.zvvnmod.meco-rust-android:<tag>")`.

## Why this shape

The C ABI is the universal server interop (PHP/Go/Java all load it); UniFFI gives idiomatic
Swift/Kotlin for native apps; wasm covers the web. Every consumer runs the same Java-verified core,
so behavior can never drift across platforms — the problem the old per-language ports had.
