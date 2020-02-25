use criterion::{criterion_group, Criterion};

pub fn inner(c: &mut Criterion) {
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("virt_detect", |b| {
        b.iter(|| rt.block_on(heim::virt::detect()))
    });
}

criterion_group!(bench, inner);
