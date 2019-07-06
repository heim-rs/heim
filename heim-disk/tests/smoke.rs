#![feature(async_await)]

use heim_common::prelude::*;
use heim_disk as disk;

#[runtime::test]
async fn smoke_partitions() {
    let mut partitions = disk::partitions();
    while let Some(part) = partitions.next().await {
        let part = part.unwrap();

        let _ = part.device();
        let _ = part.mount_point();
        let _ = part.file_system();
    }
}

#[runtime::test]
async fn smoke_partitions_physical() {
    let mut partitions = disk::partitions_physical();
    while let Some(part) = partitions.next().await {
        let part = part.unwrap();

        let _ = part.device();
        let _ = part.mount_point();
        let _ = part.file_system();
    }
}

#[runtime::test]
async fn smoke_usage() {
    let usage = disk::usage("/").await;

    let usage = usage.unwrap();

    let _ = usage.total();
    let _ = usage.used();
    let _ = usage.free();
    let _ = usage.ratio();
}

#[heim_derive::skip_ci(target_os = "windows")]
#[runtime::test]
async fn smoke_io_counters() {
    let mut counters = disk::io_counters();
    while let Some(count) = counters.next().await {
        let count = count.unwrap();

        let _ = count.device_name();
        let _ = count.read_count();
        let _ = count.write_count();
        let _ = count.read_bytes();
        let _ = count.write_bytes();
    }
}

#[runtime::test]
async fn smoke_io_counters_physical() {
    let mut counters = disk::io_counters_physical();
    while let Some(count) = counters.next().await {
        let count = count.unwrap();

        let _ = count.device_name();
        let _ = count.read_count();
        let _ = count.write_count();
        let _ = count.read_bytes();
        let _ = count.write_bytes();
    }
}
