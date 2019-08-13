use std::path::PathBuf;
use std::ffi::CStr;
use std::convert::TryFrom;

use heim_common::prelude::*;

use super::{bindings, pids};
use crate::{Pid, ProcessResult, Status};

mod cpu_times;

pub use self::cpu_times::CpuTime;

#[derive(Debug)]
pub struct Process {
    pid: Pid,
}

impl Process {
    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn parent_pid(&self) -> impl Future<Output = ProcessResult<Pid>> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => future::ok(kinfo_proc.kp_eproc.e_ppid),
            Err(e) => future::err(e),
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
            Err(e) => future::err(e),
        }
    }

    pub fn exe(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        future::ready(bindings::libproc::pid_path(self.pid))
    }

    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => {
                future::ready(Status::try_from(kinfo_proc.kp_proc.p_stat).map_err(From::from))
            },
            Err(e) => future::err(e),
        }
    }

    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        // TODO: Stub
        future::err(Error::incompatible("https://github.com/heim-rs/heim/issues/108").into())
    }
}

pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    pids()
        .map_err(Into::into)
        .map_ok(|pid| Process {
            pid,
        })
}
