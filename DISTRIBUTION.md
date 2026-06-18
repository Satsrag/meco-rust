# Distributing meco — easy install for every platform

One Rust core (`meco-core`), one thin binding per ecosystem, and one release pipeline
(`.github/workflows/release.yml`) that builds every native artifact from a single version tag.
**Recommendation:** publish `meco-rust/` as its own Git repository (so the workflow, `Package.swift`,
and the per-language packages sit at the repo root, like `meco_php`/`meco_dart` are separate repos).

## How a consumer adds it (the goal: one line)

| Platform | Add to a project | Backed by |
|---|---|---|
| **PHP** (server) | `composer require zvvnmod/meco` | `meco-cabi` C ABI via FFI |
| **Web / Node** | `npm install meco-wasm` | `meco-wasm` (wasm-bindgen) |
| **iOS** (SwiftPM) | `.package(url: "https://github.com/zvvnmod/meco-rust-swift", from: "0.1.0")` | `meco-uniffi` (Swift) |
| **iOS** (CocoaPods) | `pod 'Meco'` | `meco-uniffi` (Swift) |
| **Android** (Gradle) | `implementation("com.zvvnmod:meco-android:0.1.0")` | `meco-uniffi` (Kotlin) |
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

A tag `vX.Y.Z` triggers the CI, which builds and uploads: per-platform `libmeco`, the iOS
`MecoFFI.xcframework`, the Android `.aar`, and the wasm npm package. Then:

### PHP → Packagist
1. Push the repo to GitHub; submit it once at https://packagist.org (auto-updates via webhook).
2. Native libs: the CI bundles `libmeco` for each platform into `bindings/php/prebuilt/<os>-<arch>/`
   and includes them in the tagged commit, so `composer require` is zero-config. Alternatives:
   set `MECO_LIB`, or run `bindings/php/scripts/build-lib.sh`.

### Web → npm
The `wasm` CI job runs `wasm-pack build … && npm publish` (needs `NPM_TOKEN`). Consumers `npm i meco-wasm`.

### iOS → SwiftPM / CocoaPods
CI builds `MecoFFI.xcframework` + the generated `meco_uniffi.swift`. For SwiftPM, point the
`binaryTarget` at the release zip (`url` + `checksum` from `swift package compute-checksum`). For
CocoaPods, `pod trunk push bindings/swift/Meco.podspec`.

### Android → Maven Central or JitPack
CI builds the AAR (`cargo ndk` → `.so` per ABI + UniFFI Kotlin + `gradlew assembleRelease`).
Publish to Maven Central (`maven-publish` + signing) for `implementation("com.zvvnmod:meco-android:…")`,
or enable JitPack for `implementation("com.github.zvvnmod.meco-rust-android:<tag>")`.

## Why this shape

The C ABI is the universal server interop (PHP/Go/Java all load it); UniFFI gives idiomatic
Swift/Kotlin for native apps; wasm covers the web. Every consumer runs the same Java-verified core,
so behavior can never drift across platforms — the problem the old per-language ports had.
