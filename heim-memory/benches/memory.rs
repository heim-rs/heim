#![feature(async_await, test)]

extern crate test;

use heim_memory as memory;

#[runtime::bench]
async fn bench_memory() {
    memory::memory().await
}

#[runtime::bench]
async fn bench_swap() {
    memory::swap().await
}
