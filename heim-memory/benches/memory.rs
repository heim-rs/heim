#![feature(test)]

extern crate test;

use heim_memory as memory;

#[heim_derive::bench]
async fn bench_memory() {
    memory::memory().await
}

#[heim_derive::bench]
async fn bench_swap() {
    memory::swap().await
}
