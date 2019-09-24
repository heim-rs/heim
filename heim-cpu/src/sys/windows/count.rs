use std::mem;
use std::ptr;

use winapi::um::{sysinfoapi, winbase, winnt};

use heim_common::prelude::*;

pub fn logical_count() -> impl Future<Output = Result<u64>> {
    // Safety: seems to be a very straightforward function.
    // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getactiveprocessorcount
    let result = unsafe { winbase::GetActiveProcessorCount(winnt::ALL_PROCESSOR_GROUPS) };

    if result > 0 {
        future::ok(u64::from(result))
    } else {
        future::err(Error::last_os_error())
    }
}

pub fn physical_count() -> impl Future<Output = Result<Option<u64>>> {
    let mut buffer_size = 0;

    let _ = unsafe {
        sysinfoapi::GetLogicalProcessorInformationEx(
            winnt::RelationProcessorCore,
            ptr::null_mut(),
            &mut buffer_size,
        )
    };

    let struct_size = mem::size_of::<winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>() as u32;
    let length = buffer_size / struct_size;
    let mut buf = Vec::with_capacity(length as usize);

    let result = unsafe {
        sysinfoapi::GetLogicalProcessorInformationEx(
            winnt::RelationProcessorCore,
            buf.as_mut_ptr(),
            &mut buffer_size,
        )
    };

    if result == 0 {
        return future::err(Error::last_os_error());
    } else {
        unsafe {
            buf.set_len(length as usize);
        }
    }

    if !buf.is_empty() {
        future::ok(Some(buf.len() as u64))
    } else {
        future::ok(None)
    }
}
