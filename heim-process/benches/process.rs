#[macro_use]
extern crate criterion;

use criterion::{Criterion};

use heim_process as process;
use heim_runtime::{self as runtime, SyncRuntime};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pids", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| {
            runtime.block_collect(process::pids())
        })
    });
    c.bench_function("pid_exists", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| {
            runtime.block_run(process::pid_exists(1))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
