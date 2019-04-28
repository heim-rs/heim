#![feature(await_macro, async_await, test)]

extern crate test;

use heim_common::prelude::*;
use heim_host as host;

#[runtime::bench]
async fn bench_platform() {
    await!(host::platform())
}

#[runtime::bench]
async fn bench_uptime() {
    await!(host::uptime())
}

#[runtime::bench]
async fn bench_users() {
    let stream = host::users().for_each(|_| future::ready(()));

    await!(stream)
}
