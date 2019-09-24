use std::cmp;
use std::ffi::OsString;
use std::hash;
use std::io;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use heim_common::prelude::*;
use heim_common::units::Time;
use winapi::um::processthreadsapi;

use super::{bindings, pid_exists, pids};
use crate::sys::common::UniqueId;
use crate::{Pid, ProcessError, ProcessResult, Status};

mod command;
mod cpu_times;
mod create_time;
mod memory;
mod suspend;

pub use self::command::{Command, CommandIter};
pub use self::cpu_times::CpuTime;
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

    pub fn parent_pid(&self) -> impl Future<Output = ProcessResult<Pid>> {
        let pid = self.pid;

        future::lazy(move |_| {
            let snapshot = bindings::snapshot::Snapshot::new()?;

            snapshot
                .flatten()
                .find(|entry| entry.th32ProcessID == pid)
                .map(|entry| Ok(entry.th32ParentProcessID))
                .unwrap_or_else(|| Err(ProcessError::NoSuchProcess(pid)))
        })
    }

    pub fn name(&self) -> impl Future<Output = ProcessResult<String>> {
        // TODO: Rewrite it when `async_await` will be stable
        let pid = self.pid;
        future::lazy(move |_| match pid {
            0 => Ok("System Idle Process".to_string()),
            4 => Ok("System".to_string()),
            _ => Err(()),
        })
        .or_else(move |_| {
            // According to `psutil` working with a `Process::exe` is faster,
            // but will fail in cases with AD and processes owned by other users.
            //
            // Also we do not care about any errors which might happen,
            // that's why `future::err(())` is used
            match bindings::ProcessHandle::query_limited_info(pid) {
                Ok(handle) => match handle.exe() {
                    Ok(path) => match path.file_name() {
                        None => future::err(()),
                        Some(name) => future::ok(name.to_string_lossy().into_owned()),
                    },
                    Err(..) => future::err(()),
                },
                Err(..) => future::err(()),
            }
        })
        .or_else(move |_| {
            let snapshot = match bindings::snapshot::Snapshot::new() {
                Ok(snapshot) => snapshot,
                Err(e) => return future::err(e.into()),
            };

            let result = snapshot
                .flatten()
                .find(|entry| entry.th32ProcessID == pid)
                .map(|entry| {
                    let first_null = entry
                        .szExeFile
                        .iter()
                        .position(|byte| *byte == 0x00)
                        .unwrap_or(0);
                    let os_str = OsString::from_wide(&entry.szExeFile[..first_null]);

                    Ok(os_str.to_string_lossy().into_owned())
                })
                .unwrap_or_else(|| Err(ProcessError::NoSuchProcess(pid)));

            future::ready(result)
        })
    }

    pub fn exe(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        let pid = self.pid;
        future::lazy(move |_| {
            // TODO: Move that check into the `bindings::ProcessHandle` constructors
            if pid == 0 || pid == 4 {
                Err(ProcessError::AccessDenied(pid))
            } else {
                let handle =
                    bindings::ProcessHandle::query_limited_info(pid).map_err(|e| {
                        match e.kind() {
                            io::ErrorKind::PermissionDenied => ProcessError::AccessDenied(pid),
                            _ => e.into(),
                        }
                    })?;

                handle.exe().map_err(|e| match e.kind() {
                    io::ErrorKind::PermissionDenied => ProcessError::AccessDenied(pid),
                    _ => e.into(),
                })
            }
        })
    }

    pub fn command(&self) -> impl Future<Output = ProcessResult<Command>> {
        self::command::command(self.pid)
    }

    pub fn cwd(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        future::err(Error::incompatible("https://github.com/heim-rs/heim/issues/105").into())
    }

    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        match suspend::is_suspended(self.pid) {
            Ok(true) => future::ok(Status::Stopped),
            Ok(false) => future::ok(Status::Running),
            Err(e) => future::err(e),
        }
    }

    pub fn create_time(&self) -> impl Future<Output = ProcessResult<Time>> {
        future::ok(self.unique_id.create_time())
    }

    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        // TODO: Move that check into the `bindings::ProcessHandle`
        if self.pid == 0 {
            future::Either::Left(future::err(ProcessError::AccessDenied(self.pid)))
        } else {
            let pid = self.pid;

            let f = future::lazy(move |_| {
                let handle = bindings::ProcessHandle::query_limited_info(pid)
                    // TODO: `query_limited_info` should return the `ProcessError`
                    .map_err(|e| {
                        match e.kind() {
                            io::ErrorKind::PermissionDenied => ProcessError::AccessDenied(pid),
                            _ => e.into(),
                        }
                    })?;

                handle.cpu_time().map_err(ProcessError::from)
            });

            future::Either::Right(f)
        }
    }

    pub fn memory(&self) -> impl Future<Output = ProcessResult<Memory>> {
        // TODO: Move that check into the `bindings::ProcessHandle`
        if self.pid == 0 {
            future::Either::Left(future::err(ProcessError::AccessDenied(self.pid)))
        } else {
            let pid = self.pid;

            let f = future::lazy(move |_| {
                let handle = bindings::ProcessHandle::query_limited_info(pid)
                    // TODO: `query_limited_info` should return the `ProcessError`
                    .map_err(|e| {
                        match e.kind() {
                            io::ErrorKind::PermissionDenied => ProcessError::AccessDenied(pid),
                            _ => e.into(),
                        }
                    })?;

                handle
                    .memory()
                    .map(Memory::from)
                    .map_err(ProcessError::from)
            });

            future::Either::Right(f)
        }
    }

    pub fn is_running(&self) -> impl Future<Output = ProcessResult<bool>> {
        let unique_id = self.unique_id.clone();
        get(self.pid).map_ok(move |other| other.unique_id == unique_id)
    }

    pub fn suspend(&self) -> impl Future<Output = ProcessResult<()>> {
        // TODO: Move that check into the `bindings::ProcessHandle`
        if self.pid == 0 {
            future::Either::Left(future::err(ProcessError::AccessDenied(self.pid)))
        } else {
            let pid = self.pid;

            let f = future::lazy(move |_| {
                let handle =
                    bindings::ProcessHandle::for_suspend_resume(pid).map_err(|e| {
                        match e.kind() {
                            io::ErrorKind::PermissionDenied => ProcessError::AccessDenied(pid),
                            _ => e.into(),
                        }
                    })?;

                handle.suspend().map_err(ProcessError::from)
            });

            future::Either::Right(f)
        }
    }

    pub fn resume(&self) -> impl Future<Output = ProcessResult<()>> {
        // TODO: Move that check into the `bindings::ProcessHandle`
        if self.pid == 0 {
            future::Either::Left(future::err(ProcessError::AccessDenied(self.pid)))
        } else {
            let pid = self.pid;

            let f = future::lazy(move |_| {
                let handle =
                    bindings::ProcessHandle::for_suspend_resume(pid).map_err(|e| {
                        match e.kind() {
                            io::ErrorKind::PermissionDenied => ProcessError::AccessDenied(pid),
                            _ => e.into(),
                        }
                    })?;

                handle.resume().map_err(ProcessError::from)
            });

            future::Either::Right(f)
        }
    }

    pub fn terminate(&self) -> impl Future<Output = ProcessResult<()>> {
        self.kill()
    }

    pub fn kill(&self) -> impl Future<Output = ProcessResult<()>> {
        // TODO: Move that check into the `bindings::ProcessHandle`
        if self.pid == 0 {
            future::Either::Left(future::err(ProcessError::AccessDenied(self.pid)))
        } else {
            let pid = self.pid;

            let f = future::lazy(move |_| {
                let handle =
                    bindings::ProcessHandle::for_termination(pid).map_err(|e| match e.kind() {
                        io::ErrorKind::PermissionDenied => ProcessError::AccessDenied(pid),
                        _ => e.into(),
                    })?;

                handle.terminate().map_err(ProcessError::from)
            });

            future::Either::Right(f)
        }
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
fn get_unchecked(pid: Pid) -> impl Future<Output = ProcessResult<Process>> {
    self::create_time::get(pid).map_ok(move |create_time| Process {
        pid,
        unique_id: UniqueId::new(pid, create_time),
    })
}

pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    pids().and_then(get_unchecked)
}

pub fn get(pid: Pid) -> impl Future<Output = ProcessResult<Process>> {
    pid_exists(pid).and_then(move |is_exists| {
        if is_exists {
            future::Either::Left(get_unchecked(pid))
        } else {
            let f = future::err(ProcessError::NoSuchProcess(pid));

            future::Either::Right(f)
        }
    })
}

pub fn current() -> impl Future<Output = ProcessResult<Process>> {
    let pid = unsafe { processthreadsapi::GetCurrentProcessId() };

    get_unchecked(pid)
}
