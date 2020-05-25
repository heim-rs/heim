use criterion::{criterion_group, Criterion};

pub fn inner(c: &mut Criterion) {
    c.bench_function("memory_memory", |b| {
        b.iter(|| smol::block_on(heim::memory::memory()))
    });

    c.bench_function("memory_swap", |b| {
        b.iter(|| smol::block_on(heim::memory::swap()))
    });
}

criterion_group!(bench, inner);
