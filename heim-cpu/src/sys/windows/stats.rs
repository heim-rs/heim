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

    match perf_info.iter().next() {
        Some(sys_info) => Ok((
            u64::from(sys_info.ContextSwitches),
            u64::from(sys_info.SystemCalls),
        )),
        None => unreachable!("NtQuerySystemInformation did not returned any information"),
    }
}

fn dpc_count() -> Result<u64> {
    let info: Vec<winternl::SYSTEM_INTERRUPT_INFORMATION> = winternl::query_system_information()?;

    let count = info.into_iter().fold(0, |acc, item| acc + item.DpcCount);

    Ok(count.into())
}

fn interrupts() -> Result<u64> {
    let info: Vec<winternl::SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION> =
        winternl::query_system_information()?;

    let count = info
        .into_iter()
        .fold(0, |acc, item| acc + item.InterruptCount);

    Ok(count.into())
}

pub fn stats() -> impl Future<Output = Result<CpuStats>> {
    future::lazy(|_| {
        let (ctx_switches, system_calls) = system_performance_info()?;
        let dpc = dpc_count()?;
        let interrupts = interrupts()?;

        Ok(CpuStats {
            ctx_switches,
            system_calls,
            interrupts,
            dpc_count: dpc,
        })
    })
}
