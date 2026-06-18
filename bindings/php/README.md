# zvvnmod/meco (PHP)

Mongolian Encoding Converter for PHP — a thin **FFI** binding to the Rust `meco` core, verified
byte-exact against the original Java (`200/200` on the shared golden corpus). One engine for PHP,
servers, web and mobile instead of a hand-maintained PHP reimplementation.

## Install

```bash
composer require zvvnmod/meco
```

Requires `php >= 7.4` with `ext-ffi` enabled (`ffi.enable=1`, or preload the library in production).

## Use

```php
use Meco\Meco;

echo Meco::translate(Meco::Z52, Meco::MENK_SHAPE, $z52Input);
echo Meco::translate(Meco::DELEHI, Meco::ZVVNMOD, $unicodeInput);
echo Meco::version();
```

Encodings: `Meco::ZVVNMOD`, `Meco::DELEHI`, `Meco::MENK_SHAPE`, `Meco::MENK_LETTER`, `Meco::Z52`.
`translate()` throws `RuntimeException` on an unknown encoding or unsupported conversion.

## The native library

The wrapper loads `libmeco` (the `meco-cabi` C ABI). It is resolved in this order:

1. `MECO_LIB` env var (absolute path to the lib), else
2. `prebuilt/<os>-<arch>/libmeco.{so,dylib,dll}` shipped in the package (filled by the release CI
   for `linux-x86_64`, `linux-aarch64`, `darwin-aarch64`, `darwin-x86_64`, `windows-x86_64`), else
3. the bare name `libmeco.{ext}` from the system loader path.

To build it locally for your platform (drops it into `prebuilt/`):

```bash
bash bindings/php/scripts/build-lib.sh
```

### Production note

With `ffi.enable=preload`, declare the library in `opcache.preload` for best performance and to
allow FFI outside the CLI. The wrapper uses `FFI::cdef` (definitions inline), so no separate header
file is needed; for preloading you can switch to `FFI::load` of a `.h` if you prefer.

## Replacing the old PHP port

This package is a drop-in for the hand-written `meco_php`: same conversions, but backed by the
single Java-verified Rust core, so it can't drift. Swap `Meco\TranslateService::translate(...)`
calls for `Meco\Meco::translate(...)` (note the argument order is `from, to, input`).
