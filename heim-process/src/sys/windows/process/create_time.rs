use winapi::shared::ntdef;
use winapi::shared::winerror;

use heim_common::sys::IntoTime;
use heim_common::units::Time;

use crate::sys::windows::bindings;
use crate::{Pid, ProcessError, ProcessResult};

const ERROR_ACCESS_DENIED: i32 = winerror::ERROR_ACCESS_DENIED as i32;
const ERROR_PRIVILEGE_NOT_HELD: i32 = winerror::ERROR_PRIVILEGE_NOT_HELD as i32;

fn traverse(pid: Pid) -> ProcessResult<Time> {
    let pid_handle = pid as ntdef::HANDLE;
    let processes = bindings::processes::NtProcesses::load()?;
    let process = processes
        .iter()
        .find(|process| process.process.UniqueProcessId == pid_handle)
        .ok_or(ProcessError::NoSuchProcess(pid))?;

    Ok(process.process.CreateTime.into_time())
}

pub async fn get(pid: Pid) -> ProcessResult<Time> {
    if pid == 0 || pid == 4 {
        return heim_host::boot_time().await.map_err(Into::into);
    }

    let handle = bindings::ProcessHandle::query_limited_info(pid)?;

    match handle.create_time() {
        Ok(time) => Ok(time),
        Err(ProcessError::Load(e)) => {
            match e {
                // TODO: DRY
                _ if e.raw_os_error() == Some(libc::EPERM) => traverse(pid),
                _ if e.raw_os_error() == Some(libc::EACCES) => traverse(pid),
                _ if e.raw_os_error() == Some(ERROR_ACCESS_DENIED) => traverse(pid),
                _ if e.raw_os_error() == Some(ERROR_PRIVILEGE_NOT_HELD) => traverse(pid),
                _ => Err(ProcessError::Load(e)),
            }
        }
        Err(e) => Err(e),
    }
}
