use criterion::{criterion_group, Criterion};
use futures::prelude::*;

pub fn inner(c: &mut Criterion) {
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("net_io_counters", |b| {
        b.iter(|| {
            let stream = heim::net::io_counters().for_each(|_| async {});
            rt.block_on(stream)
        })
    });

    c.bench_function("net_nic", |b| {
        b.iter(|| {
            let stream = heim::net::nic().for_each(|_| async {});
            rt.block_on(stream)
        })
    });
}

criterion_group!(bench, inner);
