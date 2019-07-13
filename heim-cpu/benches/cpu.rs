#![feature(async_await, test)]

extern crate test;

use heim_common::prelude::*;
use heim_cpu as cpu;

#[runtime::bench]
async fn bench_frequency() {
    cpu::frequency().await
}

#[runtime::bench]
async fn bench_logical_count() {
    cpu::logical_count().await
}

#[runtime::bench]
async fn bench_physical_count() {
    cpu::physical_count().await
}

#[runtime::bench]
async fn bench_stats() {
    cpu::stats().await
}

#[runtime::bench]
async fn bench_time() {
    cpu::time().await
}

#[runtime::bench]
async fn bench_times() {
    let stream = cpu::times().for_each(|_| future::ready(()));

    stream.await
}
