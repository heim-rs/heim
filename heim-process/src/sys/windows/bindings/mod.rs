use std::io::{Error, Result};
use std::mem;

use winapi::shared::minwindef::DWORD;
use winapi::um::psapi;

pub mod handle;
pub mod processes;
pub mod snapshot;

pub use self::handle::ProcessHandle;

pub fn pids() -> Result<Vec<DWORD>> {
    let mut processes = Vec::with_capacity(1024);
    let mut bytes_returned: DWORD = 0;

    loop {
        let cb = (processes.capacity() * mem::size_of::<DWORD>()) as DWORD;
        let result =
            unsafe { psapi::EnumProcesses(processes.as_mut_ptr(), cb, &mut bytes_returned) };

        if result == 0 {
            return Err(Error::last_os_error());
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
