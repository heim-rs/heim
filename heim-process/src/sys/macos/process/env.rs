use std::io;

use crate::sys::macos::{pid_exists, wrappers};
use crate::sys::unix::Environment;
use crate::{Pid, ProcessError, ProcessResult};

pub async fn environment(pid: Pid) -> ProcessResult<Environment> {
    match wrappers::ProcArgs::get(pid) {
        Ok(proc_args) => Ok(proc_args.environment()),
        Err(e) if e.raw_os_error() == Some(libc::EINVAL) => {
            if pid_exists(pid).await? {
                Err(ProcessError::ZombieProcess(pid))
            } else {
                Err(e.into())
            }
        }
        Err(e) if e.as_inner().kind() == io::ErrorKind::PermissionDenied => {
            Err(ProcessError::AccessDenied(pid))
        }
        Err(e) => Err(e.into()),
    }
}
