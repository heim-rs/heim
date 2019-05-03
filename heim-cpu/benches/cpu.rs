#![allow(stable_features)]
#![feature(await_macro, async_await, futures_api, test)]

extern crate test;

use heim_common::prelude::*;
use heim_cpu as cpu;

#[runtime::bench]
async fn bench_time() {
    await!(cpu::time());
}

#[runtime::bench]
async fn bench_times() {
    let stream = cpu::times().for_each(|_| future::ready(()));

    await!(stream);
}

#[runtime::bench]
async fn bench_stats() {
    let stats = cpu::stats();

    await!(stats);
}

#[runtime::bench]
async fn bench_frequency() {
    await!(cpu::frequency());
}
