use criterion::{criterion_group, Criterion};

pub fn inner(c: &mut Criterion) {
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("memory_memory", |b| {
        b.iter(|| rt.block_on(heim::memory::memory()))
    });

    c.bench_function("memory_swap", |b| {
        b.iter(|| rt.block_on(heim::memory::swap()))
    });
}

criterion_group!(bench, inner);
