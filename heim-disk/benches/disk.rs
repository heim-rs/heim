#![feature(async_await, test)]

extern crate test;

use heim_common::prelude::*;
use heim_disk as disk;

#[cfg(unix)]
static USAGE_PATH: &'static str = "/";

#[cfg(windows)]
static USAGE_PATH: &'static str = "C:\\";

#[runtime::bench]
async fn bench_partitions() {
    let stream = disk::partitions().for_each(|_| future::ready(()));

    stream.await
}

#[runtime::bench]
async fn bench_partitions_physical() {
    let stream = disk::partitions_physical().for_each(|_| future::ready(()));

    stream.await
}

#[runtime::bench]
async fn bench_io_counters() {
    let stream = disk::io_counters().for_each(|_| future::ready(()));

    stream.await
}

#[runtime::bench]
async fn bench_io_counters_physical() {
    let stream = disk::io_counters_physical().for_each(|_| future::ready(()));

    stream.await
}

#[runtime::bench]
async fn bench_usage() {
    disk::usage(USAGE_PATH).await
}
