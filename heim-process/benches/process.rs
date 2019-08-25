#![feature(async_closure, test)]

extern crate test;

use heim_common::prelude::*;
use heim_process as process;

#[heim_derive::bench]
async fn bench_pids() {
    let stream = process::pids().for_each(|_| future::ready(()));

    stream.await
}

#[heim_derive::bench]
async fn bench_plain_processes() {
    let stream = process::processes().for_each(|_| future::ready(()));

    stream.await
}

#[heim_derive::bench]
async fn bench_processes_with_extra() {
    let stream = process::processes()
        .try_for_each(async move |process| {
            process.parent_pid().await?;
            process.name().await?;

            Ok(())
        })
        .into_stream()
        .for_each(|_| future::ready(()));

    stream.await
}
