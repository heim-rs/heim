use std::cmp;
use std::hash;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
use heim_common::units::Time;
use heim_runtime::fs;

use super::{pid_exists, pids};
use crate::os::linux::IoCounters;
use crate::os::unix::Signal;
use crate::sys::common::UniqueId;
use crate::sys::unix::pid_kill;
use crate::{Pid, ProcessError, ProcessResult, Status};

mod procfs;

pub use self::procfs::{Command, CommandIter, CpuTime, Memory, Stat};

#[derive(Debug)]
pub struct Process {
    pid: Pid,
    unique_id: UniqueId,
}

impl Process {
    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub async fn parent_pid(&self) -> ProcessResult<Pid> {
        let Stat { ppid, .. } = procfs::stat(self.pid).await?;

        Ok(ppid)
    }

    pub async fn name(&self) -> ProcessResult<String> {
        let Stat { name, .. } = procfs::stat(self.pid).await?;

        // Process name is truncated for Linux,
        // so we are using command line to fetch the full process name

        // TODO: Move `15` to the const
        if name.len() < 15 {
            return Ok(name);
        }

        let command = procfs::command(self.pid).await?;
        // There might be an absolute path to executable
        let path = command
            .into_iter()
            .next()
            .map(Path::new)
            .and_then(Path::file_name);

        match path {
            // We can assume that on Linux paths and filenames are UTF-8,
            // and since OsStr does not has the `starts_with` method,
            // we could compare raw bytes
            Some(exe) if exe.as_bytes().starts_with(name.as_bytes()) => {
                Ok(exe.to_string_lossy().into_owned())
            }
            _ => Ok(name),
        }
    }

    pub async fn exe(&self) -> ProcessResult<PathBuf> {
        match fs::read_link(format!("/proc/{}/exe", self.pid)).await {
            Ok(path) => Ok(path),
            Err(..) => match pid_exists(self.pid).await {
                Ok(true) => Ok(PathBuf::new()),
                _ => Err(ProcessError::ZombieProcess(self.pid)),
            },
        }
    }

    pub async fn command(&self) -> ProcessResult<Command> {
        self::procfs::command(self.pid).await
    }

    pub async fn cwd(&self) -> ProcessResult<PathBuf> {
        match fs::read_link(format!("/proc/{}/cwd", self.pid)).await {
            Ok(path) => Ok(path),
            Err(..) => match pid_exists(self.pid).await {
                Ok(true) => Err(ProcessError::ZombieProcess(self.pid)),
                _ => Err(ProcessError::AccessDenied(self.pid)),
            },
        }
    }

    pub async fn status(&self) -> ProcessResult<Status> {
        let Stat { state, .. } = procfs::stat(self.pid).await?;

        Ok(state)
    }

    pub async fn create_time(&self) -> ProcessResult<Time> {
        Ok(self.unique_id.create_time())
    }

    pub async fn cpu_time(&self) -> ProcessResult<CpuTime> {
        Ok(procfs::stat(self.pid).await?.into())
    }

    pub async fn memory(&self) -> ProcessResult<Memory> {
        procfs::stat_memory(self.pid).await
    }

    pub async fn is_running(&self) -> ProcessResult<bool> {
        let other = get(self.pid).await?;

        Ok(other.unique_id == self.unique_id)
    }

    pub async fn signal(&self, signal: Signal) -> ProcessResult<()> {
        if self.is_running().await? {
            pid_kill(self.pid, signal)
        } else {
            Err(ProcessError::NoSuchProcess(self.pid))
        }
    }

    pub async fn suspend(&self) -> ProcessResult<()> {
        self.signal(Signal::Stop).await
    }

    pub async fn resume(&self) -> ProcessResult<()> {
        self.signal(Signal::Cont).await
    }

    pub async fn terminate(&self) -> ProcessResult<()> {
        self.signal(Signal::Term).await
    }

    pub async fn kill(&self) -> ProcessResult<()> {
        self.signal(Signal::Kill).await
    }

    // Linux-specific methods

    pub async fn io_counters(&self) -> ProcessResult<IoCounters> {
        procfs::io(self.pid).await
    }

    pub fn net_io_counters(&self) -> BoxStream<ProcessResult<heim_net::IoCounters>> {
        heim_net::os::linux::io_counters_for_pid(self.pid())
            .map_err(Error::from)  // TODO: TEMPORARY
            .map_err(ProcessError::from)
            .boxed()
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
    pids().map_err(Into::into).and_then(get)
}

pub async fn get(pid: Pid) -> ProcessResult<Process> {
    let Stat { create_time, .. } = procfs::stat(pid).await?;

    Ok(Process {
        pid,
        unique_id: UniqueId::new(pid, create_time),
    })
}

pub fn current() -> impl Future<Output = ProcessResult<Process>> {
    let pid = unsafe { libc::getpid() };

    get(pid)
}
