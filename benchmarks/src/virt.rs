use criterion::{criterion_group, Criterion};

pub fn inner(c: &mut Criterion) {
    c.bench_function("virt_detect", |b| {
        b.iter(|| smol::block_on(heim::virt::detect()))
    });
}

criterion_group!(bench, inner);
