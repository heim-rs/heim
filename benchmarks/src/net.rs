use criterion::{criterion_group, Criterion};
use futures::prelude::*;

pub fn inner(c: &mut Criterion) {
    c.bench_function("net_io_counters", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::net::io_counters().await?.for_each(|_| async {}).await;

                Ok::<(), heim::Error>(())
            })
        })
    });

    c.bench_function("net_nic", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::net::nic().await?.for_each(|_| async {}).await;

                Ok::<(), heim::Error>(())
            })
        })
    });
}

criterion_group!(bench, inner);
