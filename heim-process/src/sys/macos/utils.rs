use std::convert::TryFrom;

use super::bindings;
use crate::{Pid, ProcessError, Status};

pub(crate) fn catch_zombie<T: Into<ProcessError>>(e: T, pid: Pid) -> ProcessError {
    match e.into() {
        ProcessError::Load(ref e) if e.raw_os_error() == Some(libc::ESRCH) => {
            let kinfo_proc = match bindings::process(pid) {
                Ok(info) => info,
                Err(e) => return e,
            };

            match Status::try_from(kinfo_proc.kp_proc.p_stat) {
                Ok(Status::Zombie) => ProcessError::ZombieProcess(pid),
                Ok(_) => ProcessError::AccessDenied(pid),
                Err(e) => e.into(),
            }
        }
        other => other,
    }
}
