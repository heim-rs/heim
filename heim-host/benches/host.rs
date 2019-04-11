#[macro_use]
extern crate criterion;

use criterion::Criterion;

use heim_host as host;
use heim_runtime::{self as runtime, SyncRuntime};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("platform", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_run(host::platform()))
    });
    c.bench_function("uptime", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_run(host::uptime()))
    });
    c.bench_function("users", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_collect(host::users()).count())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
