#![feature(async_await, test)]

use heim_common::prelude::*;
use heim_cpu as cpu;

#[heim_derive::skip_ci(target_os = "linux")]
#[runtime::test]
async fn smoke_frequency() {
    let freq = cpu::frequency().await;

    let freq = freq.unwrap();
    assert!(freq.current().get() > 0);
    let _ = freq.min();
    let _ = freq.max();
}

#[runtime::test]
#[cfg(target_os = "linux")]
async fn smoke_frequencies() {
    let mut frequencies = cpu::os::linux::frequencies();
    while let Some(freq) = frequencies.next().await {
        let f = freq.unwrap();

        let _ = f.current();
        let _ = f.min();
        let _ = f.max();
    }
}

//#[heim_derive::skip_ci(all())]
#[runtime::test]
async fn smoke_stats() {
    let stats = cpu::stats().await;
    let stats = stats.unwrap();

    let _ = stats.ctx_switches();
    let _ = stats.interrupts();
}

#[runtime::test]
async fn smoke_time() {
    let time = cpu::time().await;
    let time = time.unwrap();

    let _ = time.system();
    let _ = time.user();
    let _ = time.idle();
}

#[runtime::test]
async fn smoke_times() {
    let mut times = cpu::times();
    while let Some(time) = times.next().await {
        let time = time.unwrap();

        let _ = time.system();
        let _ = time.user();
        let _ = time.idle();
    }
}
