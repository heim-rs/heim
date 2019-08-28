#![feature(test)]

use heim_common::prelude::*;
use heim_common::units::frequency;
use heim_cpu as cpu;

#[heim_derive::skip_ci(target_os = "linux")]
#[heim_derive::test]
async fn smoke_frequency() {
    let freq = cpu::frequency().await.unwrap();

    assert!(freq.current().get::<frequency::hertz>() > 0);
    let _ = freq.min();
    let _ = freq.max();
}

#[heim_derive::test]
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

#[heim_derive::test]
async fn smoke_stats() {
    let stats = cpu::stats().await.unwrap();

    let _ = stats.ctx_switches();
    let _ = stats.interrupts();

    #[cfg(target_os = "linux")]
    {
        use heim_cpu::os::linux::CpuStatsExt;

        let _ = stats.soft_interrupts();
    }

    #[cfg(target_os = "macos")]
    {
        use heim_cpu::os::macos::CpuStatsExt;

        let _ = stats.soft_interrupts();
        let _ = stats.syscalls();
    }

    #[cfg(target_os = "windows")]
    {
        use heim_cpu::os::windows::CpuStatsExt;

        let _ = stats.dpc();
        let _ = stats.syscalls();
    }
}

#[heim_derive::test]
async fn smoke_time() {
    let time = cpu::time().await;
    let time = time.unwrap();

    let _ = time.system();
    let _ = time.user();
    let _ = time.idle();

    #[cfg(target_os = "linux")]
    {
        use heim_cpu::os::linux::CpuTimeExt;

        let _ = time.nice();
        let _ = time.io_wait();
        let _ = time.irq();
        let _ = time.soft_irq();
        let _ = time.steal();
        let _ = time.guest();
        let _ = time.guest_nice();
    }
}

#[heim_derive::test]
async fn smoke_times() {
    let mut times = cpu::times();
    while let Some(time) = times.next().await {
        let time = time.unwrap();

        let _ = time.system();
        let _ = time.user();
        let _ = time.idle();

        #[cfg(target_os = "linux")]
        {
            use heim_cpu::os::linux::CpuTimeExt;

            let _ = time.nice();
            let _ = time.io_wait();
            let _ = time.irq();
            let _ = time.soft_irq();
            let _ = time.steal();
            let _ = time.guest();
            let _ = time.guest_nice();
        }
    }
}

#[heim_derive::test]
async fn smoke_cpu_logical_count() {
    let count = cpu::logical_count().await.unwrap();

    assert!(count > 0);
}

// TODO: Crashes for some reasons in Azure VM, should be investigated
#[heim_derive::skip_ci(target_os = "windows")]
#[heim_derive::test]
async fn smoke_cpu_physical_count() {
    let count = cpu::physical_count().await;
    assert!(count.is_ok(), "cpu::physical_count failed: {:#?}", count);
    let count = count.unwrap();

    if let Some(cpus) = count {
        assert!(cpus > 0);
    }
}
