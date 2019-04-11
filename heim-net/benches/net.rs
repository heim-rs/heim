#[macro_use]
extern crate criterion;

use criterion::Criterion;

use heim_net as net;
use heim_runtime::{self as runtime, SyncRuntime};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("connections", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| {
            runtime
                .block_collect(net::connections(net::ConnectionKind::all()))
                .count()
        })
    });
    c.bench_function("io_counters", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_collect(net::io_counters()).count())
    });
    c.bench_function("nic", |b| {
        let mut runtime = runtime::new().unwrap();
        b.iter(|| runtime.block_collect(net::nic()).count())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
