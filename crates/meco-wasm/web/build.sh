#!/usr/bin/env bash
# Build the browser demo into a self-contained directory that can be dropped onto any static host.
#
#   crates/meco-wasm/web/build.sh [OUT_DIR]     # default OUT_DIR: crates/meco-wasm/web/dist
#
# Produces OUT_DIR/{index.html, pkg/, fonts/}. Fonts are the per-encoding fonts the demo needs to
# render Menksoft/Z52 PUA text; they are fetched from the meco font repo rather than committed here.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../../.." && pwd)"
out="${1:-$here/dist}"

echo "==> cargo build (wasm32-unknown-unknown, release)"
cargo build --manifest-path "$root/Cargo.toml" -p meco-wasm --target wasm32-unknown-unknown --release

echo "==> wasm-bindgen --target web"
rm -rf "$out/pkg"
wasm-bindgen --target web --no-typescript \
    --out-dir "$out/pkg" \
    "$root/target/wasm32-unknown-unknown/release/meco_wasm.wasm"

echo "==> fonts"
mkdir -p "$out/fonts"
fetch() { # fetch <url-path> <dest-name>
    [ -f "$out/fonts/$2" ] || curl -sfL "https://raw.githubusercontent.com/Satsrag/meco/HEAD/$1" -o "$out/fonts/$2"
}
fetch "fonts/delehi/mnglwhiteotf.ttf"              delehi.ttf
fetch "fonts/menk/MQG8103.ttf"                     menk_letter.ttf
fetch "fonts/menk/MenksoftQagan_shape.ttf"         menk_shape.ttf
fetch "fonts/z52/7%20-%20Z52%20Tsagaan%20Tig.otf"  z52.otf
# zvvnmod.ttf is generated from fonts/zvvnmod/zvvnmod.sfd; the built copy lives on the tools site.
[ -f "$out/fonts/ZvvnMod.ttf" ] || \
    curl -sfL "https://raw.githubusercontent.com/Satsrag/satsrag.github.io/HEAD/mapping/assets/zvvnmod.ttf" \
         -o "$out/fonts/ZvvnMod.ttf"

cp "$here/index.html" "$out/index.html"
echo "==> done: $out"
echo "    preview:  python3 -m http.server -d '$out' 8000"
