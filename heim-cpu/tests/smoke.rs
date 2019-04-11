use heim_common::units::si::frequency::hertz;
use heim_cpu as cpu;
use heim_runtime::{self as runtime, SyncRuntime};

#[test]
#[heim_derive::skip_ci]
fn smoke_frequency() {
    let mut rt = runtime::new().unwrap();
    let freq = rt.block_run(cpu::frequency());

    assert!(freq.is_ok());
    let freq = freq.unwrap();
    assert!(freq.current().get::<hertz>() > 0);
    let _ = freq.min();
    let _ = freq.max();
}

#[test]
#[cfg(target_os = "linux")]
fn smoke_frequencies() {
    let mut rt = runtime::new().unwrap();
    let freq = rt.block_collect(cpu::os::linux::frequencies());

    for f in freq.flatten() {
        let _ = f.current();
        let _ = f.min();
        let _ = f.max();
    }
}

#[test]
#[heim_derive::skip_ci]
fn smoke_stats() {
    let mut rt = runtime::new().unwrap();
    let stats = rt.block_run(cpu::stats());

    assert!(stats.is_ok());
    let stats = stats.unwrap();

    let _ = stats.ctx_switches();
    let _ = stats.interrupts();
}

#[test]
fn smoke_time() {
    let mut rt = runtime::new().unwrap();
    let time = rt.block_run(cpu::time());

    assert!(time.is_ok());
    let time = time.unwrap();

    let _ = time.system();
    let _ = time.user();
    let _ = time.idle();
}

#[test]
fn smoke_times() {
    let mut rt = runtime::new().unwrap();
    let times = rt.block_collect(cpu::times());

    for time in times.flatten() {
        let _ = time.system();
        let _ = time.user();
        let _ = time.idle();
    }
}
