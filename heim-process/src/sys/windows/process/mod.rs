use std::cmp;
use std::ffi::OsString;
use std::hash;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use heim_common::prelude::*;
use heim_common::units::Time;
use heim_host::User;
use winapi::um::processthreadsapi;

use super::{bindings, pid_exists, pids};
use crate::sys::common::UniqueId;
use crate::{Pid, ProcessError, ProcessResult, Status};

mod command;
mod cpu_times;
mod create_time;
mod env;
mod memory;
mod suspend;

pub use self::command::{Command, CommandIter};
pub use self::cpu_times::CpuTime;
pub use self::env::{Environment, EnvironmentIter, IntoEnvironmentIter};
pub use self::memory::Memory;

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
        let snapshot = bindings::snapshot::Snapshot::new()?;

        snapshot
            .flatten()
            .find(|entry| entry.th32ProcessID == self.pid)
            .map(|entry| Ok(entry.th32ParentProcessID))
            .unwrap_or_else(|| Err(ProcessError::NoSuchProcess(self.pid)))
    }

    pub async fn name(&self) -> ProcessResult<String> {
        let res = match self.pid {
            0 => Ok("System Idle Process".to_string()),
            4 => Ok("System".to_string()),
            _ => Err(()),
        };

        res.or_else(|_| {
            // According to `psutil` working with a `Process::exe` is faster,
            // but will fail in cases with AD and processes owned by other users.
            match bindings::ProcessHandle::query_limited_info(self.pid) {
                Ok(handle) => match handle.exe() {
                    Ok(path) => match path.file_name() {
                        None => Err(()),
                        Some(name) => Ok(name.to_string_lossy().into_owned()),
                    },
                    Err(..) => Err(()),
                },
                Err(..) => Err(()),
            }
        })
        .or_else(|_| {
            let snapshot = bindings::snapshot::Snapshot::new()?;

            snapshot
                .flatten()
                .find(|entry| entry.th32ProcessID == self.pid)
                .map(|entry| {
                    let first_null = entry
                        .szExeFile
                        .iter()
                        .position(|byte| *byte == 0x00)
                        .unwrap_or(0);
                    let os_str = OsString::from_wide(&entry.szExeFile[..first_null]);

                    Ok(os_str.to_string_lossy().into_owned())
                })
                .unwrap_or_else(|| Err(ProcessError::NoSuchProcess(self.pid)))
        })
    }

    pub async fn exe(&self) -> ProcessResult<PathBuf> {
        if self.pid == 0 || self.pid == 4 {
            Err(ProcessError::AccessDenied(self.pid))
        } else {
            let handle = bindings::ProcessHandle::query_limited_info(self.pid)?;

            handle.exe()
        }
    }

    pub async fn command(&self) -> ProcessResult<Command> {
        self::command::command(self.pid)
    }

    pub async fn cwd(&self) -> ProcessResult<PathBuf> {
        unimplemented!("https://github.com/heim-rs/heim/issues/105")
    }

    pub async fn status(&self) -> ProcessResult<Status> {
        if suspend::is_suspended(self.pid)? {
            Ok(Status::Stopped)
        } else {
            Ok(Status::Running)
        }
    }

    pub async fn environment(&self) -> ProcessResult<Environment> {
        unimplemented!()
    }

    pub async fn create_time(&self) -> ProcessResult<Time> {
        Ok(self.unique_id.create_time())
    }

    pub async fn cpu_time(&self) -> ProcessResult<CpuTime> {
        // TODO: Move that check into the `bindings::ProcessHandle`
        if self.pid == 0 {
            Err(ProcessError::AccessDenied(self.pid))
        } else {
            let handle = bindings::ProcessHandle::query_limited_info(self.pid)?;

            handle.cpu_time()
        }
    }

    pub async fn memory(&self) -> ProcessResult<Memory> {
        // TODO: Move that check into the `bindings::ProcessHandle`?
        if self.pid == 0 {
            Err(ProcessError::AccessDenied(self.pid))
        } else {
            let handle = bindings::ProcessHandle::query_limited_info(self.pid)?;

            handle.memory().map(Memory::from)
        }
    }

    pub async fn user(&self) -> ProcessResult<User> {
        if self.pid == 0 || self.pid == 4 {
            Err(ProcessError::AccessDenied(self.pid))
        } else {
            let handle = bindings::ProcessHandle::query_limited_info(self.pid)?;

            handle.owner()
        }
    }

    pub async fn is_running(&self) -> ProcessResult<bool> {
        let other = get(self.pid).await?;

        Ok(other == *self)
    }

    pub async fn suspend(&self) -> ProcessResult<()> {
        let handle = bindings::ProcessHandle::for_suspend_resume(self.pid)?;

        handle.suspend().map_err(Into::into)
    }

    pub async fn resume(&self) -> ProcessResult<()> {
        let handle = bindings::ProcessHandle::for_suspend_resume(self.pid)?;

        handle.resume().map_err(Into::into)
    }

    pub async fn terminate(&self) -> ProcessResult<()> {
        self.kill().await
    }

    pub async fn kill(&self) -> ProcessResult<()> {
        let handle = bindings::ProcessHandle::for_termination(self.pid)?;

        handle.terminate().map_err(Into::into)
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

/// Create the `Process` from `pid` without checking first if pid is alive.
async fn get_unchecked(pid: Pid) -> ProcessResult<Process> {
    let create_time = self::create_time::get(pid).await?;

    Ok(Process {
        pid,
        unique_id: UniqueId::new(pid, create_time),
    })
}

pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    pids().and_then(get_unchecked)
}

pub async fn get(pid: Pid) -> ProcessResult<Process> {
    if pid_exists(pid).await? {
        get_unchecked(pid).await
    } else {
        Err(ProcessError::NoSuchProcess(pid))
    }
}

pub async fn current() -> ProcessResult<Process> {
    let pid = unsafe { processthreadsapi::GetCurrentProcessId() };

    get_unchecked(pid).await
}
