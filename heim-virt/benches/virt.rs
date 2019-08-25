#![feature(test)]

extern crate test;

use heim_virt as virt;

#[heim_derive::bench]
async fn bench_detect() {
    virt::detect().await
}
