#![feature(async_await, test)]

extern crate test;

use heim_common::prelude::*;
use heim_host as host;

#[runtime::bench]
async fn bench_platform() {
    host::platform().await
}

#[runtime::bench]
async fn bench_uptime() {
    host::uptime().await
}

#[runtime::bench]
async fn bench_users() {
    let stream = host::users().for_each(|_| future::ready(()));

    stream.await
}
