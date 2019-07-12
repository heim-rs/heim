use std::mem;
use std::result;

use heim_common::prelude::*;

use winapi::shared::minwindef::DWORD;
use winapi::um::psapi;

use crate::{Pid, ProcessError};

pub fn pids() -> impl Stream<Item = result::Result<Pid, ProcessError>> {
    future::lazy(|_| {
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
                return Err(Error::last_os_error().into())
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

        Ok(stream::iter(processes).map(Ok))
    })
    .try_flatten_stream()
    .map_ok(Pid::from)
}

pub fn pid_exists(_pid: Pid) -> impl Future<Output = result::Result<bool, ProcessError>> {
    // TODO: Stub
    future::ok(false)
}
