use std::path::PathBuf;

use heim_common::prelude::*;
use heim_common::units::Time;
use heim_runtime::fs;

use super::pids;
use crate::{Pid, Status, ProcessError, ProcessResult};

mod procfs;

pub use self::procfs::{Memory, CpuTime};

#[derive(Debug)]
pub struct Process {
    pid: Pid,
    create_time: Time,
}

impl Process {
    pub fn get(pid: Pid) -> impl Future<Output = ProcessResult<Self>> {
        procfs::stat(pid)
            .map_ok(move |procfs::Stat { create_time, .. } | Process {
                pid,
                create_time,
            })
    }

    pub fn current() -> impl Future<Output = ProcessResult<Self>> {
        future::lazy(|_| {
            unsafe {
                libc::getpid()
            }
        })
        .then(|pid| {
            Process::get(pid)
        })
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn parent_pid(&self) -> impl Future<Output = ProcessResult<Pid>> {
        procfs::stat(self.pid).map_ok(|procfs::Stat { ppid, .. }| ppid)
    }

    pub fn name(&self) -> impl Future<Output = ProcessResult<String>> {
        procfs::stat(self.pid).map_ok(|procfs::Stat { name, .. }| name)
    }

    pub fn exe(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        let pid = self.pid; // Hello borrow checker, my old friend

        fs::read_link(format!("/proc/{}/exe", self.pid)).or_else(move |_| {
            fs::path_exists(format!("/proc/{}", pid)).then(move |exists| {
                if exists {
                    future::ok(PathBuf::new())
                } else {
                    future::err(ProcessError::ZombieProcess(pid))
                }
            })
        })
    }

    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        procfs::stat(self.pid).map_ok(|procfs::Stat { state, .. } | state)
    }

    pub fn create_time(&self) -> impl Future<Output = ProcessResult<Time>> {
        future::ok(self.create_time)
    }

    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        procfs::stat(self.pid).map_ok(Into::into)
    }

    pub fn memory(&self) -> impl Future<Output = ProcessResult<Memory>> {
        procfs::stat_memory(self.pid)
    }
}

pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    pids()
        .map_err(Into::into)
        .and_then(|pid| self::procfs::stat(pid).map_ok(move |stat| (pid, stat)))
        .map_ok(|(pid, stat)| Process {
            pid,
            create_time: stat.create_time,
        })
}
