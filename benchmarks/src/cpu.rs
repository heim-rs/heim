use criterion::{criterion_group, Criterion};
use futures::prelude::*;

fn inner(c: &mut Criterion) {
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("cpu_logical_count", |b| {
        b.iter(|| rt.block_on(heim::cpu::logical_count()))
    });

    c.bench_function("cpu_physical_count", |b| {
        b.iter(|| rt.block_on(heim::cpu::physical_count()))
    });

    c.bench_function("cpu_frequency", |b| {
        b.iter(|| rt.block_on(heim::cpu::frequency()))
    });

    c.bench_function("cpu_stats", |b| b.iter(|| rt.block_on(heim::cpu::stats())));

    c.bench_function("cpu_time", |b| b.iter(|| rt.block_on(heim::cpu::time())));

    c.bench_function("cpu_times", |b| {
        b.iter(|| {
            let stream = heim::cpu::times().for_each(|_| async {});
            rt.block_on(stream)
        })
    });
}

criterion_group!(bench, inner);
