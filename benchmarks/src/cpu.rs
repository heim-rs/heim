use criterion::{criterion_group, Criterion};
use futures::prelude::*;

fn inner(c: &mut Criterion) {
    c.bench_function("cpu_logical_count", |b| {
        b.iter(|| smol::block_on(heim::cpu::logical_count()))
    });

    c.bench_function("cpu_physical_count", |b| {
        b.iter(|| smol::block_on(heim::cpu::physical_count()))
    });

    c.bench_function("cpu_frequency", |b| {
        b.iter(|| smol::block_on(heim::cpu::frequency()))
    });

    c.bench_function("cpu_stats", |b| {
        b.iter(|| smol::block_on(heim::cpu::stats()))
    });

    c.bench_function("cpu_time", |b| b.iter(|| smol::block_on(heim::cpu::time())));

    c.bench_function("cpu_times", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::cpu::times().await?.for_each(|_| async {}).await;
                Ok::<(), heim::Error>(())
            })
        })
    });
}

criterion_group!(bench, inner);
