use wasmtime::{Engine, Module};

pub struct WasmBuilder<'a> {
    wasm_bytes: &'a [u8],
}

impl<'a> WasmBuilder<'a> {
    pub fn new(wasm_bytes: &'a [u8]) -> Self {
        let wasm_bytes = wasm_bytes;
        Self { wasm_bytes }
    }

    pub fn build(&self) -> Result<Module, wasmtime::Error> {
        let engine = Engine::default();
        Module::new(&engine, &self.wasm_bytes)
    }
}
