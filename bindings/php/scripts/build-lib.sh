#!/usr/bin/env bash
# Build libmeco (the meco-cabi C ABI) for the current host and place it where the PHP wrapper
# looks: bindings/php/prebuilt/<os>-<arch>/libmeco.<ext>.
set -euo pipefail

# repo root = meco-rust/ (two levels up from bindings/php/scripts)
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"

cargo build -p meco-cabi --release

case "$(uname -s)" in
  Darwin) os=darwin; ext=dylib ;;
  Linux)  os=linux;  ext=so ;;
  MINGW*|MSYS*|CYGWIN*) os=windows; ext=dll ;;
  *) echo "unsupported OS: $(uname -s)" >&2; exit 1 ;;
esac
case "$(uname -m)" in
  arm64|aarch64) arch=aarch64 ;;
  x86_64|amd64)  arch=x86_64 ;;
  *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;;
esac

dest="bindings/php/prebuilt/$os-$arch"
mkdir -p "$dest"
cp "target/release/libmeco.$ext" "$dest/"
echo "installed $dest/libmeco.$ext"
