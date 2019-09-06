use std::cmp;
use std::hash;
use std::path::PathBuf;

use futures::future::BoxFuture;
use futures::stream::BoxStream;

use heim_common::prelude::*;
use heim_common::units::Time;
use heim_runtime::fs;

use crate::{Pid, Status, ProcessError, ProcessResult};
use crate::sys::common::UniqueId;
use crate::sys::unix::pid_kill;
use crate::os::linux::IoCounters;
use crate::os::unix::Signal;
use super::{pids, pid_exists};

mod procfs;

pub use self::procfs::{Memory, CpuTime, Command, CommandIter};

#[derive(Debug)]
pub struct Process {
    pid: Pid,
    unique_id: UniqueId,
}

impl Process {
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
            pid_exists(pid)
                .and_then(move |exists| {
                    if exists {
                        future::ok(PathBuf::new())
                    } else {
                        future::err(ProcessError::ZombieProcess(pid))
                    }
                })
        })
    }

    pub fn command(&self) -> impl Future<Output = ProcessResult<Command>> {
        let pid = self.pid;

        self::procfs::command(pid)
    }

    pub fn cwd(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        let pid = self.pid;

        fs::read_link(format!("/proc/{}/cwd", self.pid))
            .or_else(move |_| {
                pid_exists(pid)
                    .and_then(move |exists| {
                        if exists {
                            future::err(ProcessError::ZombieProcess(pid))
                        } else {
                            future::err(ProcessError::AccessDenied(pid))
                        }
                    })
            })
    }

    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        procfs::stat(self.pid).map_ok(|procfs::Stat { state, .. } | state)
    }

    pub fn create_time(&self) -> impl Future<Output = ProcessResult<Time>> {
        future::ok(self.unique_id.create_time())
    }

    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        procfs::stat(self.pid).map_ok(Into::into)
    }

    pub fn memory(&self) -> impl Future<Output = ProcessResult<Memory>> {
        procfs::stat_memory(self.pid)
    }

    pub fn is_running(&self) -> impl Future<Output = ProcessResult<bool>> {
        let unique_id = self.unique_id.clone();
        get(self.pid).map_ok(move |other| {
            other.unique_id == unique_id
        })
    }

    pub fn signal(&self, signal: Signal) -> BoxFuture<ProcessResult<()>> {
        let pid = self.pid;
        self.is_running()
            .and_then(move |is_running| {
                if is_running {
                    future::ready(pid_kill(pid, signal))
                } else {
                    future::err(ProcessError::NoSuchProcess(pid))
                }
            })
            .boxed()
    }

    // Linux-specific methods

    pub fn io_counters(&self) -> BoxFuture<ProcessResult<IoCounters>> {
        procfs::io(self.pid).boxed()
    }

    pub fn net_io_counters(&self) -> BoxStream<ProcessResult<heim_net::IoCounters>> {
        heim_net::os::linux::io_counters_for_pid(self.pid()).map_err(Into::into).boxed()
    }
}

impl hash::Hash for Process {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.unique_id.hash(state);
    }
}

impl cmp::PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.unique_id == other.unique_id
    }
}

impl cmp::Eq for Process {}

pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    pids()
        .map_err(Into::into)
        .and_then(get)
}

pub fn get(pid: Pid) -> impl Future<Output = ProcessResult<Process>> {
    procfs::stat(pid)
        .map_ok(move |procfs::Stat { create_time, .. } | Process {
            pid,
            unique_id: UniqueId::new(pid, create_time),
        })
}

pub fn current() -> impl Future<Output = ProcessResult<Process>> {
    future::lazy(|_| {
        unsafe {
            libc::getpid()
        }
    })
    .then(get)
}

