use std::hint::black_box;

use wasmtime::{Engine, Module};

fn create_module() -> Result<Module, wasmtime::Error> {
    let engine = Engine::default();
    Module::new(&engine, include_bytes!("../../test.wasm"))
}

const ITERATIONS: usize = 1000;

fn bench_creation() {
    // warmup
    let module = create_module();
    assert!(module.is_ok());
    let mut modules: Vec<Module> = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        modules.push(black_box(create_module().unwrap()));
    }
    assert_eq!(modules.len(), ITERATIONS);

    // benchmark
    let mut modules: Vec<Module> = Vec::with_capacity(ITERATIONS);
    let now = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        modules.push(black_box(create_module().unwrap()));
    }
    let elapsed = now.elapsed();
    assert_eq!(modules.len(), ITERATIONS);
    let avg_duration = elapsed.as_micros() as f64 / ITERATIONS as f64;
    println!("Average creation duration: {} microseconds", avg_duration);
}

fn bench_destruction() {
    let mut modules = Vec::with_capacity(1000);
    for _ in 0..1000 {
        let module = create_module().unwrap();
        //modules.push(MyModule { module });
        modules.push(module);
    }
    assert_eq!(modules.len(), ITERATIONS);

    // benchmark
    let now = std::time::Instant::now();
    drop(black_box(modules));
    let elapsed = now.elapsed();
    let avg_duration = elapsed.as_micros() as f64 / ITERATIONS as f64;
    println!(
        "Average destruction duration: {} microseconds",
        avg_duration
    );
}

fn main() {
    bench_creation();
    bench_destruction();
}
