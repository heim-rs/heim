use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use heim_common::prelude::*;

use super::{pids, bindings};
use crate::{Pid, ProcessError, ProcessResult, Status};

mod cpu_times;
mod suspend;

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
        future::lazy(move |_| {
            match pid {
                0 => Ok("System Idle Process".to_string()),
                4 => Ok("System".to_string()),
                _ => Err(())
            }
        })
        .or_else(move |_| {
            // According to `psutil` working with a `Process::exe` is faster,
            // but will fail in cases with AD and processes owned by other users.
            //
            // Also we do not care about any errors which might happen,
            // that's why `future::err(())` is used
            match bindings::ProcessHandle::query_limited_info(pid) {
                Ok(handle) => {
                    match handle.exe() {
                        Ok(path) => {
                            match path.file_name() {
                                None => future::err(()),
                                Some(name) => future::ok(name.to_string_lossy().into_owned())
                            }
                        }
                        Err(..) => future::err(()),
                    }
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
                    let first_null = entry.szExeFile.iter()
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
            if pid == 0 || pid == 4 {
                Err(ProcessError::AccessDenied(pid))
            } else {
                let handle = bindings::ProcessHandle::query_limited_info(pid)?;

                handle.exe().map_err(ProcessError::from)
            }
        })
    }

    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        match suspend::is_suspended(self.pid) {
            Ok(true) => future::ok(Status::Stopped),
            Ok(false) => future::ok(Status::Running),
            Err(e) => future::err(e),
        }
    }

    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        // TODO: Stub
        future::err(Error::incompatible("https://github.com/heim-rs/heim/issues/109").into())
    }
}

pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    pids()
        .map_ok(|pid| Process {
            pid,
        })
}
