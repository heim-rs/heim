use std::cmp;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::hash;
use std::io;
use std::path::PathBuf;

use futures::future::BoxFuture;

use heim_common::prelude::*;
use heim_common::sys::IntoTime;
use heim_common::units::Time;

use super::{bindings, pids, utils::catch_zombie};
use crate::os::unix::Signal;
use crate::sys::common::UniqueId;
use crate::sys::unix::pid_kill;
use crate::{Pid, ProcessError, ProcessResult, Status};

mod command;
mod cpu_times;
mod memory;

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
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => future::ok(kinfo_proc.kp_eproc.e_ppid),
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn name(&self) -> impl Future<Output = ProcessResult<String>> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => {
                let raw_str = unsafe { CStr::from_ptr(kinfo_proc.kp_proc.p_comm.as_ptr()) };
                let name = raw_str.to_string_lossy().into_owned();

                future::ok(name)
            }
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

    pub fn command(&self) -> impl Future<Output = ProcessResult<Command>> {
        self::command::command(self.pid)
    }

    pub fn cwd(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        match darwin_libproc::pid_cwd(self.pid) {
            Ok(path) => future::ok(path),
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => {
                future::ready(Status::try_from(kinfo_proc.kp_proc.p_stat).map_err(From::from))
            }
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn create_time(&self) -> impl Future<Output = ProcessResult<Time>> {
        future::ok(self.unique_id.create_time())
    }

    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        match darwin_libproc::task_info(self.pid) {
            Ok(task_info) => future::ok(CpuTime::from(task_info)),
            Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                future::err(ProcessError::AccessDenied(self.pid))
            }
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn memory(&self) -> impl Future<Output = ProcessResult<Memory>> {
        match darwin_libproc::task_info(self.pid) {
            Ok(task_info) => future::ok(Memory::from(task_info)),
            Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                future::err(ProcessError::AccessDenied(self.pid))
            }
            Err(e) => future::err(catch_zombie(e, self.pid)),
        }
    }

    pub fn is_running(&self) -> impl Future<Output = ProcessResult<bool>> {
        let unique_id = self.unique_id.clone();
        get(self.pid).map_ok(move |other| other.unique_id == unique_id)
    }

    // `Self::signal` needs to return `BoxFuture`,
    // but the `Self::kill` does not
    fn _signal(&self, signal: Signal) -> impl Future<Output = ProcessResult<()>> {
        let pid = self.pid;

        self.is_running().and_then(move |is_running| {
            if is_running {
                future::ready(pid_kill(pid, signal))
            } else {
                future::err(ProcessError::NoSuchProcess(pid))
            }
        })
    }

    pub fn signal(&self, signal: Signal) -> BoxFuture<ProcessResult<()>> {
        self._signal(signal).boxed()
    }

    pub fn suspend(&self) -> impl Future<Output = ProcessResult<()>> {
        self._signal(Signal::Stop)
    }

    pub fn resume(&self) -> impl Future<Output = ProcessResult<()>> {
        self._signal(Signal::Cont)
    }

    pub fn terminate(&self) -> impl Future<Output = ProcessResult<()>> {
        self._signal(Signal::Term)
    }

    pub fn kill(&self) -> impl Future<Output = ProcessResult<()>> {
        self._signal(Signal::Kill)
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

pub fn get(pid: Pid) -> impl Future<Output = ProcessResult<Process>> {
    match bindings::process(pid) {
        Ok(kinfo_proc) => {
            let create_time = unsafe {
                // TODO: How can it be guaranteed that in this case
                // `p_un.p_starttime` will be filled correctly?
                kinfo_proc.kp_proc.p_un.p_starttime
            };
            let create_time = create_time.into_time();
            debug_assert!(!create_time.is_nan());

            future::ok(Process {
                pid,
                unique_id: UniqueId::new(pid, create_time),
            })
        }
        Err(e) => future::err(catch_zombie(e, pid)),
    }
}

pub fn current() -> impl Future<Output = ProcessResult<Process>> {
    future::lazy(|_| unsafe { libc::getpid() }).then(get)
}
