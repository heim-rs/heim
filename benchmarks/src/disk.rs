use criterion::{criterion_group, Criterion};
use futures::prelude::*;

#[cfg(unix)]
static USAGE_PATH: &str = "/";

#[cfg(windows)]
static USAGE_PATH: &str = "C:\\";

pub fn inner(c: &mut Criterion) {
    c.bench_function("disk_io_counters", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::disk::io_counters()
                    .await?
                    .for_each(|_| async {})
                    .await;

                Ok::<(), heim::Error>(())
            })
        })
    });

    c.bench_function("disk_io_counters_physical", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::disk::io_counters_physical()
                    .await?
                    .for_each(|_| async {})
                    .await;

                Ok::<(), heim::Error>(())
            })
        })
    });

    c.bench_function("disk_partitions", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::disk::partitions().await?.for_each(|_| async {}).await;

                Ok::<(), heim::Error>(())
            })
        })
    });

    c.bench_function("disk_partitions_physical", |b| {
        b.iter(|| {
            smol::block_on(async {
                heim::disk::partitions_physical()
                    .await?
                    .for_each(|_| async {})
                    .await;

                Ok::<(), heim::Error>(())
            })
        })
    });

    c.bench_function("disk_usage", |b| {
        b.iter(|| {
            let fut = heim::disk::usage(USAGE_PATH);
            smol::block_on(fut)
        })
    });
}

criterion_group!(bench, inner);
