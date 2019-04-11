#[macro_use]
extern crate criterion;

use criterion::Criterion;

use heim_memory as memory;
use heim_runtime::{self as runtime, SyncRuntime};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("memory", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_run(memory::memory()))
    });
    c.bench_function("swap", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_run(memory::swap()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
