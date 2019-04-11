#[macro_use]
extern crate criterion;

use criterion::Criterion;

use heim_runtime::{self as runtime, SyncRuntime};
use heim_disk as disk;

#[cfg(unix)]
static USAGE_PATH: &'static str = "/";

#[cfg(windows)]
static USAGE_PATH: &'static str = "C:\\";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("partitions", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| {
            runtime.block_collect(disk::partitions()).count()
        })
    });
    c.bench_function("partitions_physical", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| {
            runtime.block_collect(disk::partitions_physical()).count()
        })
    });
    c.bench_function("usage", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| {
            runtime.block_run(disk::usage(USAGE_PATH))
        })
    });
    c.bench_function("io_counters", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| {
            runtime.block_collect(disk::io_counters()).count()
        })
    });
    c.bench_function("io_counters_physical", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| {
            runtime.block_collect(disk::io_counters_physical()).count()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
