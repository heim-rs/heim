#![feature(async_await, test)]

extern crate test;

use heim_virt as virt;

#[runtime::bench]
async fn bench_detect() {
    virt::detect().await
}
