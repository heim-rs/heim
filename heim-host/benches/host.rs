#![feature(test)]

extern crate test;

use heim_common::prelude::*;
use heim_host as host;

#[heim_derive::bench]
async fn bench_platform() {
    host::platform().await
}

#[heim_derive::bench]
async fn bench_uptime() {
    host::uptime().await
}

#[heim_derive::bench]
async fn bench_boot_time() {
    host::boot_time().await
}

#[heim_derive::bench]
async fn bench_users() {
    let stream = host::users().for_each(|_| future::ready(()));

    stream.await
}
