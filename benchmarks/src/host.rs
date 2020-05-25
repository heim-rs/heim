use criterion::{criterion_group, Criterion};
use futures::prelude::*;

pub fn inner(c: &mut Criterion) {
    c.bench_function("host_platform", |b| {
        b.iter(|| smol::block_on(heim::host::platform()))
    });

    c.bench_function("host_uptime", |b| {
        b.iter(|| smol::block_on(heim::host::uptime()))
    });

    c.bench_function("host_boot_time", |b| {
        b.iter(|| smol::block_on(heim::host::boot_time()))
    });

    c.bench_function("host_users", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::host::users().await?.for_each(|_| async {}).await;

                Ok::<(), heim::Error>(())
            })
        })
    });
}

criterion_group!(bench, inner);
