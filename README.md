# wasmtime-test

This is a test repository to check the deallocation time of a `module` in `wasmtime`.

## How to run

```bash
cargo build --target wasm32-unknown-unknown --release

cargo build --release

cargo run --release --manifest-path runner/Cargo.toml
```