use std::env::current_dir;

use lazy_static::lazy_static;
use wasm::WasmBuilder;
use wasmtime::Module;

mod wasm;

lazy_static! {
    static ref WASM_BYTES: Vec<u8> = read_wasm_bytes();
}

fn read_wasm_bytes() -> Vec<u8> {
    let wasm_path = current_dir()
        .expect("cannot get current directory")
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("wasmtime_test.wasm");
    std::fs::read(&wasm_path).unwrap()
}

fn bench_creation(wasm_bytes: &'static [u8], iterations: usize) -> Vec<Module> {
    // warmup
    for _ in 0..iterations {
        let builder = WasmBuilder::new(&wasm_bytes);
        let extension = builder.build();
        assert!(extension.is_ok());
    }

    let sample = WasmBuilder::new(&wasm_bytes).build().unwrap();
    let mut modules = vec![sample; iterations];
    // benchmark
    let now = std::time::Instant::now();
    for wasm in modules.iter_mut() {
        *wasm = WasmBuilder::new(&wasm_bytes).build().unwrap();
    }
    let elapsed = now.elapsed();
    let avg_duration = elapsed.as_micros() as f64 / iterations as f64;
    println!("Average creation duration: {} microseconds", avg_duration);

    modules
}

fn bench_destruction(iterations: usize, extensions: Vec<Module>) {
    // benchmark
    let now = std::time::Instant::now();
    for extension in extensions {
        drop(extension);
    }
    let elapsed = now.elapsed();
    let avg_duration = elapsed.as_micros() as f64 / iterations as f64;
    println!(
        "Average destruction duration: {} microseconds",
        avg_duration
    );
}

fn main() {
    let iterations = 1000;
    let wasm_modules = bench_creation(&WASM_BYTES, iterations);
    bench_destruction(iterations, wasm_modules);
}
