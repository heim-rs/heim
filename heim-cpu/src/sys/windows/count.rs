use winapi::um::{winbase, winnt};

use super::wrappers::count::LogicalProcessors;
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
    match LogicalProcessors::get() {
        Ok(processors) => {
            let count = processors
                .iter()
                .filter(|p| p.Relationship == winnt::RelationProcessorCore)
                .count();

            if count > 0 {
                future::ok(Some(count as u64))
            } else {
                future::ok(None)
            }
        }
        Err(e) => future::err(e.into()),
    }
}
