#![feature(async_await, test)]

extern crate test;

use heim_net as net;

#[runtime::bench]
async fn bench_io_counters() {
    let stream = net::io_counters().for_each(|_| future::ready(()));

    stream.await
}

#[runtime::bench]
async fn bench_nic() {
    let stream = net::nic().for_each(|_| future::ready(()));

    stream.await
}
