#![feature(test)]

extern crate test;

use heim_common::prelude::*;
use heim_cpu as cpu;

#[heim_derive::bench]
async fn bench_frequency() {
    cpu::frequency().await
}

#[heim_derive::bench]
async fn bench_logical_count() {
    cpu::logical_count().await
}

#[heim_derive::bench]
async fn bench_physical_count() {
    cpu::physical_count().await
}

#[heim_derive::bench]
async fn bench_stats() {
    cpu::stats().await
}

#[heim_derive::bench]
async fn bench_time() {
    cpu::time().await
}

#[heim_derive::bench]
async fn bench_times() {
    let stream = cpu::times().for_each(|_| future::ready(()));

    stream.await
}
