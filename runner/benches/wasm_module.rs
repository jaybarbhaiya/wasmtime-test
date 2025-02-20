use criterion::{black_box, criterion_group, criterion_main};
use wasmtime::{Engine, Module};

fn create_module() -> Module {
    let engine = Engine::default();
    Module::new(&engine, include_bytes!("../../test.wasm")).unwrap()

}

fn wasm_module(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("wasm_module");
    group.bench_function("creation", |b| {
        b.iter_with_large_drop(|| {
            black_box(create_module());
        })
    });

    group.bench_function("destruction", |b| {
        b.iter_batched(
            || create_module(),
            |module| {
                drop(black_box(module));
            },
            criterion::BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(benches, wasm_module);
criterion_main!(benches);
