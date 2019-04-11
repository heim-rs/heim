#[macro_use]
extern crate criterion;

use criterion::Criterion;

use heim_cpu as cpu;
use heim_runtime::{self as runtime, SyncRuntime};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("time", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_run(cpu::time()))
    });
    c.bench_function("times", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_collect(cpu::times()))
    });
    c.bench_function("stats", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_run(cpu::stats()))
    });
    c.bench_function("frequency", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_run(cpu::frequency()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
