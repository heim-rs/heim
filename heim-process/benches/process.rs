#![allow(stable_features)]
#![feature(async_await, futures_api, test)]

extern crate test;

use heim_common::prelude::*;
use heim_process as process;

#[runtime::bench]
async fn bench_pids() {
    let stream = process::pids().for_each(|_| future::ready(()));

    stream.await
}

