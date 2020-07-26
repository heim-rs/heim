use heim_common::prelude::*;

use super::bindings::winternl;

#[derive(Debug)]
pub struct CpuStats {
    ctx_switches: u64,
    interrupts: u64,
    dpc_count: u64,
    system_calls: u64,
}

impl CpuStats {
    pub fn ctx_switches(&self) -> u64 {
        self.ctx_switches
    }

    pub fn interrupts(&self) -> u64 {
        self.interrupts
    }

    pub fn dpc(&self) -> u64 {
        self.dpc_count
    }

    pub fn syscalls(&self) -> u64 {
        self.system_calls
    }
}

fn system_performance_info() -> Result<(u64, u64)> {
    let perf_info: Vec<winternl::SYSTEM_PERFORMANCE_INFORMATION> =
        winternl::query_system_information()?;

    match perf_info.get(0) {
        Some(sys_info) => Ok((
            u64::from(sys_info.ContextSwitches),
            u64::from(sys_info.SystemCalls),
        )),
        None => unreachable!("NtQuerySystemInformation did not returned any information"),
    }
}

fn dpc_count() -> Result<u64> {
    let info: Vec<winternl::SYSTEM_INTERRUPT_INFORMATION> = winternl::query_system_information()?;

    let count = info.into_iter().fold(0u64, |acc, item| {
        // TODO: Log the overflow (`info` level?)
        acc.overflowing_add(item.DpcCount.into()).0
    });

    Ok(count)
}

fn interrupts() -> Result<u64> {
    let info: Vec<winternl::SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION> =
        winternl::query_system_information()?;

    let count = info.into_iter().fold(0u64, |acc, item| {
        // `InterruptCount` type is `u32` (`ULONG`) and working with `u32`
        // in here can overflow really quick (see #250).
        // `u64` will not overflow that fast, but we still want
        // to handle that case.
        // TODO: Log the overflow (`info` level?)
        acc.overflowing_add(item.InterruptCount.into()).0
    });

    Ok(count)
}

pub async fn stats() -> Result<CpuStats> {
    let (ctx_switches, system_calls) = system_performance_info()?;
    let dpc = dpc_count()?;
    let interrupts = interrupts()?;

    Ok(CpuStats {
        ctx_switches,
        system_calls,
        interrupts,
        dpc_count: dpc,
    })
}
