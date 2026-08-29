# meco-utn57-command

Desktop/server adapter that composes the portable `meco-core` hub conversion
with the backend-neutral `zvvnmod-utn57` facade:

```text
supported meco encoding → ZVVNMOD → canonical UTN #57
```

It is intentionally a separate crate. `meco-core`, `meco-wasm`, `meco-uniffi`,
and `meco-cabi` remain pure/native and do not start Python or another process.

## Runtime setup

The current `zvvnmod-utn57` implementation uses the external `mongol-norm`
command backend. Install and verify it once on the desktop/server host:

```sh
cargo install zvvnmod-utn57 --version '=0.1.0-alpha.2'
zvvnmod-install-mongol-norm
```

Adding this adapter as a Rust dependency does not install that second runtime.

## Rust API

```rust,no_run
use meco_core::CodeType;
use meco_utn57_command::translate_to_utn57;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let input = "\u{1824}\u{180b}\u{1824}";
let output = translate_to_utn57(CodeType::Delehi, input)?;
# let _ = output;
# Ok(())
# }
```

The public error boundary distinguishes portable meco routing failures from
backend-neutral UTN #57 normalization failures. Callers do not depend on the
concrete command implementation.