#![feature(test)]

extern crate test;

use heim_common::prelude::*;
use heim_net as net;

#[heim_derive::bench]
async fn bench_io_counters() {
    let stream = net::io_counters().for_each(|_| future::ready(()));

    stream.await
}

#[heim_derive::bench]
async fn bench_nic() {
    let stream = net::nic().for_each(|_| future::ready(()));

    stream.await
}
