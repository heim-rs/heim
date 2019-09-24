use heim_common::prelude::*;

use super::bindings;

#[derive(Debug)]
pub struct CpuStats {
    ctx_switches: u64,
    interrupts: u64,
    soft_interrupts: u64,
    syscalls: u64,
    traps: u64,
}

impl CpuStats {
    pub fn ctx_switches(&self) -> u64 {
        self.ctx_switches
    }

    pub fn interrupts(&self) -> u64 {
        self.interrupts
    }

    pub fn soft_interrupts(&self) -> u64 {
        self.soft_interrupts
    }

    pub fn syscalls(&self) -> u64 {
        self.syscalls
    }

    pub fn traps(&self) -> u64 {
        self.traps
    }
}

impl From<bindings::vmmeter> for CpuStats {
    fn from(vm: bindings::vmmeter) -> CpuStats {
        CpuStats {
            ctx_switches: u64::from(vm.v_swtch),
            interrupts: u64::from(vm.v_intr),
            soft_interrupts: u64::from(vm.v_soft),
            syscalls: u64::from(vm.v_syscall),
            traps: u64::from(vm.v_trap),
        }
    }
}

pub fn stats() -> impl Future<Output = Result<CpuStats>> {
    future::lazy(|_| {
        let vm = unsafe { bindings::vm_meter()? };

        Ok(vm.into())
    })
}
