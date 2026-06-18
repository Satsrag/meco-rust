# meco-wasm — WebAssembly binding (web / Node / edge)

wasm-bindgen binding over `meco-core`, producing an npm package for the browser, Node, Deno/Bun and
edge runtimes (Cloudflare Workers, etc.). API:

```js
import init, { translate, version } from "meco-wasm";  // bundler/web target
// or: const { translate, version } = require("./pkg/meco_wasm.js");  // nodejs target
translate("z52", "menk_shape", input); // -> String; throws on unknown encoding / unsupported path
```

`from`/`to` are canonical encoding names: `zvvnmod`, `delehi`, `menk_shape`, `menk_letter`, `z52`.

## Status

Verified on Node (wasm-bindgen `--target nodejs`): **200/200 byte-exact** vs the Java golden corpus
across Z52↔MenkShape, Delehi↔Z52, Menk_Letter→Delehi, Delehi→Menk_Letter.

## Build

**With wasm-pack** (recommended — emits a ready npm package incl. `package.json`):

```sh
wasm-pack build crates/meco-wasm --target web      # browsers / bundlers
wasm-pack build crates/meco-wasm --target nodejs   # Node (CommonJS)
wasm-pack build crates/meco-wasm --target bundler  # webpack/vite/rollup
```

**With wasm-bindgen-cli** (what CI used here; pin the CLI to the `wasm-bindgen` crate version):

```sh
cargo build -p meco-wasm --target wasm32-unknown-unknown --release
cargo install wasm-bindgen-cli --version 0.2.125   # match Cargo.lock
wasm-bindgen --target nodejs --out-dir crates/meco-wasm/pkg \
  target/wasm32-unknown-unknown/release/meco_wasm.wasm
node crates/meco-wasm/test/node_verify.js          # 200/200 byte-exact
```

The generated `pkg/` is git-ignored (reproducible). `wasm-opt` (binaryen) can shrink the ~210 KB
`.wasm` further. Publish `pkg/` to npm as usual.
