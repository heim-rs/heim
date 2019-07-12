use std::mem;
use std::io::{Result, Error};

use winapi::um::{psapi, winnt, processthreadsapi, handleapi};
use winapi::shared::minwindef::DWORD;

use crate::Pid;

pub fn pids() -> Result<Vec<DWORD>> {
    let mut processes = Vec::with_capacity(1024);
    let mut bytes_returned: DWORD = 0;

    loop {
        let cb = (processes.capacity() * mem::size_of::<DWORD>()) as DWORD;
        let result = unsafe {
            psapi::EnumProcesses(
                processes.as_mut_ptr(),
                cb,
                &mut bytes_returned,
            )
        };

        if result == 0 {
            return Err(Error::last_os_error())
        }

        if cb == bytes_returned {
            processes.reserve(1024);
            continue;
        } else {
            unsafe {
                processes.set_len(bytes_returned as usize / mem::size_of::<DWORD>());
            }
            break;
        }
    }

    Ok(processes)
}

#[derive(Debug)]
pub struct Process(winnt::HANDLE);

impl Process {
    pub fn info(pid: Pid) -> Result<Process> {
        let handle = unsafe {
            processthreadsapi::OpenProcess(
                winnt::PROCESS_QUERY_INFORMATION | winnt::PROCESS_VM_READ,
                0,
                pid
            )
        };

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(Process(handle))
        }
    }

    pub fn exit_code(&self) -> Result<DWORD> {
        let mut code: DWORD = 0;

        let result = unsafe {
            processthreadsapi::GetExitCodeProcess(self.0, &mut code)
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(code)
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let result = unsafe {
            handleapi::CloseHandle(self.0)
        };

        debug_assert!(result != 0);
    }
}
