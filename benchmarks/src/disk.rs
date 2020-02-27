use criterion::{criterion_group, Criterion};
use futures::prelude::*;

#[cfg(unix)]
static USAGE_PATH: &str = "/";

#[cfg(windows)]
static USAGE_PATH: &str = "C:\\";

pub fn inner(c: &mut Criterion) {
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("disk_io_counters", |b| {
        b.iter(|| {
            let stream = heim::disk::io_counters().for_each(|_| async {});
            rt.block_on(stream)
        })
    });

    c.bench_function("disk_io_counters_physical", |b| {
        b.iter(|| {
            let stream = heim::disk::io_counters_physical().for_each(|_| async {});
            rt.block_on(stream)
        })
    });

    c.bench_function("disk_partitions", |b| {
        b.iter(|| {
            let stream = heim::disk::partitions().for_each(|_| async {});
            rt.block_on(stream)
        })
    });

    c.bench_function("disk_partitions_physical", |b| {
        b.iter(|| {
            let stream = heim::disk::partitions_physical().for_each(|_| async {});
            rt.block_on(stream)
        })
    });

    c.bench_function("disk_usage", |b| {
        b.iter(|| {
            let stream = heim::disk::usage(USAGE_PATH);
            rt.block_on(stream)
        })
    });
}

criterion_group!(bench, inner);
