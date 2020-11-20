use std::cmp;
use std::hash;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
use heim_common::units::Time;
use heim_runtime as rt;

use super::{pid_exists, pids};
use crate::os::unix::Signal;
use crate::sys::common::UniqueId;
use crate::sys::linux::process::procfs::process_file_path;
use crate::sys::unix::{pid_kill, pid_priority, pid_setpriority, pid_wait};
use crate::{Pid, ProcessError, ProcessResult, Status};

mod procfs;

pub use self::procfs::{Command, CommandIter, CpuTime, Environment, IoCounters, Memory};

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
        let procfs::Stat { ppid, .. } = procfs::stat(self.pid).await?;

        Ok(ppid)
    }

    pub async fn name(&self) -> ProcessResult<String> {
        let procfs::Stat { name, .. } = procfs::stat(self.pid).await?;

        // TODO: Move `15` to the const
        if name.len() >= 15 {
            let command = match procfs::command(self.pid).await {
                Ok(command) => command,
                // Reading process command might fail, so we should better fall back to what we got
                Err(..) => return Ok(name),
            };

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
        } else {
            Ok(name)
        }
    }

    pub async fn exe(&self) -> ProcessResult<PathBuf> {
        match rt::fs::read_link(process_file_path(self.pid, "exe")).await {
            Ok(path) => Ok(path),
            Err(..) => {
                // log::trace!() ?

                if pid_exists(self.pid).await? {
                    // Not enough permissions to read the symlink
                    Ok(PathBuf::new())
                } else {
                    Err(ProcessError::ZombieProcess(self.pid))
                }
            }
        }
    }

    pub async fn command(&self) -> ProcessResult<Command> {
        procfs::command(self.pid).await
    }

    pub async fn cwd(&self) -> ProcessResult<PathBuf> {
        match rt::fs::read_link(process_file_path(self.pid, "cwd")).await {
            Ok(path) => Ok(path),
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                Err(ProcessError::AccessDenied(self.pid))
            }
            Err(..) => {
                if pid_exists(self.pid).await? {
                    Err(ProcessError::ZombieProcess(self.pid))
                } else {
                    Err(ProcessError::AccessDenied(self.pid))
                }
            }
        }
    }

    pub async fn status(&self) -> ProcessResult<Status> {
        let procfs::Stat { state, .. } = procfs::stat(self.pid).await?;

        Ok(state)
    }

    pub async fn environment(&self) -> ProcessResult<Environment> {
        procfs::environment(self.pid).await
    }

    pub async fn create_time(&self) -> ProcessResult<Time> {
        Ok(self.unique_id.create_time())
    }

    pub async fn cpu_time(&self) -> ProcessResult<CpuTime> {
        procfs::stat(self.pid).await.map(Into::into)
    }

    pub async fn memory(&self) -> ProcessResult<Memory> {
        procfs::stat_memory(self.pid).await
    }

    pub async fn niceness(&self) -> ProcessResult<i32> {
        pid_priority(self.pid)
    }

    pub async fn set_niceness(&self, value: libc::c_int) -> ProcessResult<()> {
        pid_setpriority(self.pid, value)
    }

    pub async fn is_running(&self) -> ProcessResult<bool> {
        let other = get(self.pid).await?;

        Ok(other == *self)
    }

    pub async fn _signal(&self, signal: Signal) -> ProcessResult<()> {
        if self.is_running().await? {
            pid_kill(self.pid, signal)
        } else {
            Err(ProcessError::NoSuchProcess(self.pid))
        }
    }

    pub fn signal(&self, signal: Signal) -> BoxFuture<ProcessResult<()>> {
        self._signal(signal).boxed()
    }

    pub async fn suspend(&self) -> ProcessResult<()> {
        self._signal(Signal::Stop).await
    }

    pub async fn resume(&self) -> ProcessResult<()> {
        self._signal(Signal::Cont).await
    }

    pub async fn terminate(&self) -> ProcessResult<()> {
        self._signal(Signal::Term).await
    }

    pub async fn kill(&self) -> ProcessResult<()> {
        self.signal(Signal::Kill).await
    }

    pub async fn wait(&self) -> ProcessResult<()> {
        pid_wait(self.pid).await
    }

    // Linux-specific methods

    pub async fn io_counters(&self) -> ProcessResult<IoCounters> {
        procfs::io(self.pid).await
    }

    pub async fn net_io_counters(
        &self,
    ) -> ProcessResult<BoxStream<'_, ProcessResult<heim_net::IoCounters>>> {
        // TODO: Convert specific errors into ProcessResult error variants
        let stream = match heim_net::os::linux::io_counters_for_pid(self.pid()).await {
            Ok(stream) => stream,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return match self.status().await {
                    Ok(Status::Zombie) => Err(ProcessError::ZombieProcess(self.pid)),
                    _ => Err(e.into()),
                }
            }
            Err(e) => return Err(e.into()),
        };

        Ok(stream.map_err(Into::into).boxed())
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

pub async fn processes() -> Result<impl Stream<Item = ProcessResult<Process>>> {
    let pids = pids().await?;

    let stream = pids.map_err(Into::into).and_then(get);
    Ok(stream)
}

pub async fn get(pid: Pid) -> ProcessResult<Process> {
    let procfs::Stat { create_time, .. } = procfs::stat(pid).await?;

    Ok(Process {
        pid,
        unique_id: UniqueId::new(pid, create_time),
    })
}

pub async fn current() -> ProcessResult<Process> {
    let pid = unsafe { libc::getpid() };

    get(pid).await
}
