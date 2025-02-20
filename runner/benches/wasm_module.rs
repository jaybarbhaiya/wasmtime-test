use std::{env::current_dir, vec};

use criterion::{black_box, criterion_group, criterion_main};
use wasmtime::{Engine, Module};
use lazy_static::lazy_static;

lazy_static! {
    static ref WASM_BYTES: Vec<u8> = read_wasm_bytes();
}

fn read_wasm_bytes() -> Vec<u8> {
    let wasm_path = current_dir()
        .expect("cannot get current directory")
        .join("..")
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("wasmtime_test.wasm");
    std::fs::read(&wasm_path).unwrap()
}

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

fn wasm_module(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("wasm_module");
    group.bench_function("creation", |b| {
        b.iter(|| {
            let _ = black_box(WasmBuilder::new(&WASM_BYTES).build().unwrap());
        })
    });

    group.bench_function("destruction", |b| {
        b.iter_batched(|| {
            let module = WasmBuilder::new(&WASM_BYTES).build().unwrap();
            let modules = vec![module; 1000];
            modules
        }, |modules| {
            drop(modules);
        }, criterion::BatchSize::SmallInput);        
    });
    group.finish();
}

criterion_group!(benches, wasm_module);
criterion_main!(benches);