use std::path::PathBuf;
use std::ffi::CStr;
use std::convert::TryFrom;

use heim_common::prelude::*;

use super::{bindings, pids, utils::catch_zombie};
use crate::{Pid, ProcessResult, ProcessError, Status};

mod cpu_times;
mod memory;

pub use self::cpu_times::CpuTime;
pub use self::memory::Memory;

#[derive(Debug)]
pub struct Process {
    pid: Pid,
}

impl Process {
    pub fn get(pid: Pid) -> impl Future<Output = ProcessResult<Self>> {
        match bindings::process(pid) {
            Ok(..) => future::ok(Process {
                pid,
            }),
            Err(e) => future::err(catch_zombie(e, pid)),
        }
    }

    pub fn current() -> impl Future<Output = ProcessResult<Self>> {
        future::lazy(|_| {
            let pid = unsafe {
                libc::getpid()
            };

            Ok(Process {
                pid,
            })
        })
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn parent_pid(&self) -> impl Future<Output = ProcessResult<Pid>> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => future::ok(kinfo_proc.kp_eproc.e_ppid),
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn name(&self) -> impl Future<Output = ProcessResult<String>> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => {
                let raw_str = unsafe {
                    CStr::from_ptr(kinfo_proc.kp_proc.p_comm.as_ptr())
                };
                let name = raw_str.to_string_lossy().into_owned();

                future::ok(name)
            },
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn exe(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        match darwin_libproc::pid_path(self.pid) {
            Ok(path) => future::ok(path),
            Err(..) if self.pid == 0 => future::err(ProcessError::AccessDenied(self.pid)),
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn cwd(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        match darwin_libproc::pid_cwd(self.pid) {
            Ok(path) => future::ok(path),
            Err(e) => future::err(catch_zombie(e, self.pid))
        }
    }

    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => {
                future::ready(Status::try_from(kinfo_proc.kp_proc.p_stat).map_err(From::from))
            },
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        match darwin_libproc::task_info(self.pid) {
            Ok(task_info) => future::ok(CpuTime::from(task_info)),
            Err(e) => future::err(catch_zombie(e, self.pid))
        }
    }

    pub fn memory(&self) -> impl Future<Output = ProcessResult<Memory>> {
        match darwin_libproc::task_info(self.pid) {
            Ok(task_info) => future::ok(Memory::from(task_info)),
            Err(e) => future::err(catch_zombie(e, self.pid))
        }
    }
}

pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    pids()
        .map_err(Into::into)
        .map_ok(|pid| Process {
            pid,
        })
}
