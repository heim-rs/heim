use std::cmp;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::hash;
use std::io;
use std::path::PathBuf;

use ::futures::future::BoxFuture;

use heim_common::prelude::*;
use heim_common::sys::IntoTime;
use heim_common::units::Time;

use super::{bindings, pids, utils::catch_zombie};
use crate::os::unix::Signal;
use crate::sys::common::UniqueId;
use crate::sys::unix::{pid_kill, pid_priority, pid_setpriority, pid_wait};
pub use crate::sys::unix::{Environment, EnvironmentIter, IntoEnvironmentIter};
use crate::{Pid, ProcessError, ProcessResult, Status};

mod command;
mod cpu_times;
mod env;
mod io_counters;
mod memory;

pub use self::command::{Command, CommandIter};
pub use self::cpu_times::CpuTime;
pub use self::io_counters::IoCounters;
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
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => Ok(kinfo_proc.kp_eproc.e_ppid),
            Err(e) => Err(catch_zombie(e, self.pid)),
        }
    }

    pub async fn name(&self) -> ProcessResult<String> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => {
                let raw_str = unsafe { CStr::from_ptr(kinfo_proc.kp_proc.p_comm.as_ptr()) };
                let name = raw_str.to_string_lossy().into_owned();

                Ok(name)
            }
            Err(e) => Err(catch_zombie(e, self.pid)),
        }
    }

    pub async fn exe(&self) -> ProcessResult<PathBuf> {
        match darwin_libproc::pid_path(self.pid) {
            Ok(path) => Ok(path),
            Err(..) if self.pid == 0 => Err(ProcessError::AccessDenied(self.pid)),
            Err(e) => Err(catch_zombie(e, self.pid)),
        }
    }

    pub async fn command(&self) -> ProcessResult<Command> {
        self::command::command(self.pid).await
    }

    pub async fn cwd(&self) -> ProcessResult<PathBuf> {
        match darwin_libproc::pid_cwd(self.pid) {
            Ok(path) => Ok(path),
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                Err(ProcessError::AccessDenied(self.pid))
            }
            Err(e) => Err(catch_zombie(e, self.pid)),
        }
    }

    pub async fn status(&self) -> ProcessResult<Status> {
        match bindings::process(self.pid) {
            Ok(kinfo_proc) => Status::try_from(kinfo_proc.kp_proc.p_stat).map_err(From::from),
            Err(e) => Err(catch_zombie(e, self.pid)),
        }
    }

    pub async fn environment(&self) -> ProcessResult<Environment> {
        env::environment(self.pid).await
    }

    pub async fn create_time(&self) -> ProcessResult<Time> {
        Ok(self.unique_id.create_time())
    }

    pub async fn cpu_time(&self) -> ProcessResult<CpuTime> {
        match darwin_libproc::task_info(self.pid) {
            Ok(task_info) => Ok(CpuTime::from(task_info)),
            Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                Err(ProcessError::AccessDenied(self.pid))
            }
            Err(e) => Err(catch_zombie(e, self.pid)),
        }
    }

    pub async fn memory(&self) -> ProcessResult<Memory> {
        match darwin_libproc::task_info(self.pid) {
            Ok(task_info) => Ok(Memory::from(task_info)),
            Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                Err(ProcessError::AccessDenied(self.pid))
            }
            Err(e) => Err(catch_zombie(e, self.pid)),
        }
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

    // `Self::signal` needs to return `BoxFuture`,
    // but the `Self::kill` does not
    async fn _signal(&self, signal: Signal) -> ProcessResult<()> {
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
        self._signal(Signal::Kill).await
    }

    pub async fn wait(&self) -> ProcessResult<()> {
        pid_wait(self.pid).await
    }

    pub async fn io_counters(&self) -> ProcessResult<IoCounters> {
        io_counters::io(self.pid).await
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
    let inner = pids().await?;

    Ok(inner.map_err(Into::into).and_then(get))
}

pub async fn get(pid: Pid) -> ProcessResult<Process> {
    match bindings::process(pid) {
        Ok(kinfo_proc) => {
            let create_time = unsafe {
                // TODO: How can it be guaranteed that in this case
                // `p_un.p_starttime` will be filled correctly?
                kinfo_proc.kp_proc.p_un.p_starttime
            };
            let create_time = create_time.into_time();
            debug_assert!(!create_time.is_nan());

            Ok(Process {
                pid,
                unique_id: UniqueId::new(pid, create_time),
            })
        }
        Err(e) => Err(catch_zombie(e, pid)),
    }
}

pub async fn current() -> ProcessResult<Process> {
    let pid = unsafe { libc::getpid() };

    get(pid).await
}
