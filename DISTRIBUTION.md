# Distributing meco — easy install for every platform

One Rust core (`meco-core`), one thin binding per ecosystem, and one release pipeline
(`.github/workflows/release.yml`) that builds every native artifact from a single version tag.
**Recommendation:** publish `meco-rust/` as its own Git repository (so the workflow, `Package.swift`,
and the per-language packages sit at the repo root, like `meco_php`/`meco_dart` are separate repos).

## How a consumer adds it (the goal: one line)

| Platform | Add to a project | Backed by |
|---|---|---|
| **PHP** (server) | `composer require zvvnmod/meco` | `meco-cabi` C ABI via FFI |
| **Browser / web bundler** | Install `meco-wasm-web-*.tgz` from the GitHub Release | `meco-wasm` (wasm-bindgen web target) |
| **Node.js** | Install `meco-wasm-nodejs-*.tgz` from the GitHub Release | `meco-wasm` (wasm-bindgen nodejs target) |
| **iOS** (SwiftPM) | Download `MecoSwift.xcframework.zip` from the GitHub Release | `meco-uniffi` (Swift) |
| **iOS** (CocoaPods) | `pod 'Meco'` | `meco-uniffi` (Swift) |
| **Android** (Gradle) | `implementation("com.zvvnmod:meco-android:0.2.0")` | `meco-uniffi` (Kotlin) |
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

Before the first tag, the repository owner must create a protected GitHub environment named
`crates-io` and add an environment secret named `CARGO_REGISTRY_TOKEN`. Because `meco-core` has
not been published before, crates.io currently requires an API token for this initial publication.
Use a short-lived token that can create the package, protect the environment with required review
and a `v*` deployment rule, and add a repository tag ruleset that restricts creation, updates, and
deletion of `v*` tags to release maintainers. Never put the token in the repository, a PR, a tag
command, or workflow logs. After the first successful publish, replace the token-based publish step
with crates.io Trusted Publishing (GitHub OIDC plus a pinned crates.io auth action), verify that
updated workflow, and only then revoke the initial token.

The workflow builds and uploads per-platform `libmeco`, the iOS `MecoSwift.xcframework` and
`MecoC.xcframework`, the Android `.aar`, and the wasm npm tarball. The GitHub Release is created
only after crates.io publication succeeds. A rerun after a partial failure is safe: if the exact
crate version already exists, the workflow proceeds only when its crates.io checksum matches the
locally rebuilt package archive.

This workflow does not publish the PHP wrapper to Packagist, the wasm package to npm, the Apple
bindings to CocoaPods/SwiftPM, or the Android binding to Maven. It provides their versioned GitHub
Release assets; those ecosystem registries remain separate follow-up steps.

### Rust → crates.io

Rust consumers can enable the optional UTN #57 target through the normal core API:

```toml
meco-core = { version = "0.2.0", features = ["utn57-command"] }
```

The command-backed feature still requires the explicit backend setup documented in
`crates/meco-core/README.md`.

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
