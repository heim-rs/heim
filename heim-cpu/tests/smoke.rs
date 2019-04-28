#![feature(await_macro, async_await, futures_api, test)]

use heim_common::prelude::*;
use heim_common::units::si::frequency::hertz;
use heim_cpu as cpu;

#[heim_derive::skip_ci]
#[runtime::test]
async fn smoke_frequency() {
    let freq = await!(cpu::frequency());

    assert!(freq.is_ok());
    let freq = freq.unwrap();
    assert!(freq.current().get::<hertz>() > 0);
    let _ = freq.min();
    let _ = freq.max();
}

#[runtime::test]
#[cfg(target_os = "linux")]
async fn smoke_frequencies() {
    let mut frequencies = cpu::os::linux::frequencies();
    while let Some(freq) = await!(frequencies.next()) {
        assert!(freq.is_ok());
        let f = freq.unwrap();

        let _ = f.current();
        let _ = f.min();
        let _ = f.max();
    }
}

#[heim_derive::skip_ci]
#[runtime::test]
async fn smoke_stats() {
    let stats = await!(cpu::stats());

    assert!(stats.is_ok());
    let stats = stats.unwrap();

    let _ = stats.ctx_switches();
    let _ = stats.interrupts();
}

#[runtime::test]
async fn smoke_time() {
    let time = await!(cpu::time());

    assert!(time.is_ok());
    let time = time.unwrap();

    let _ = time.system();
    let _ = time.user();
    let _ = time.idle();
}

#[runtime::test]
async fn smoke_times() {
    let mut times = cpu::times();
    while let Some(time) = await!(times.next()) {
        assert!(time.is_ok());
        let time = time.unwrap();

        let _ = time.system();
        let _ = time.user();
        let _ = time.idle();
    }
}
