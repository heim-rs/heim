#![feature(await_macro, async_await, test)]

extern crate test;

use heim_memory as memory;

#[runtime::bench]
async fn bench_memory() {
    await!(memory::memory())
}

#[runtime::bench]
async fn bench_swap() {
    await!(memory::swap())
}
