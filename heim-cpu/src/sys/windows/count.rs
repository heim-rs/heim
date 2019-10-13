use std::mem;
use std::ptr;

use winapi::um::{sysinfoapi, winbase, winnt};

use heim_common::prelude::*;

pub async fn logical_count() -> Result<u64> {
    // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getactiveprocessorcount
    let result = unsafe { winbase::GetActiveProcessorCount(winnt::ALL_PROCESSOR_GROUPS) };

    if result > 0 {
        Ok(u64::from(result))
    } else {
        Err(Error::last_os_error().with_ffi("GetActiveProcessorCount"))
    }
}

pub async fn physical_count() -> Result<Option<u64>> {
    let mut buffer_size = 0;

    let _ = unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlogicalprocessorinformationex
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
        return Err(Error::last_os_error().with_ffi("GetLogicalProcessorInformationEx"));
    } else {
        unsafe {
            buf.set_len(length as usize);
        }
    }

    if !buf.is_empty() {
        Ok(Some(buf.len() as u64))
    } else {
        Ok(None)
    }
}
