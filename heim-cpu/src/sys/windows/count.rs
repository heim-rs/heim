use winapi::um::{winbase, winnt};

use super::wrappers::count::LogicalProcessors;
use heim_common::prelude::{Error, Result};

pub async fn logical_count() -> Result<u64> {
    // Safety: seems to be a very straightforward function.
    // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getactiveprocessorcount
    let result = unsafe { winbase::GetActiveProcessorCount(winnt::ALL_PROCESSOR_GROUPS) };

    if result > 0 {
        Ok(u64::from(result))
    } else {
        Err(Error::last_os_error().with_ffi("GetActiveProcessorCount"))
    }
}

pub async fn physical_count() -> Result<Option<u64>> {
    let processors = LogicalProcessors::get()?;
    let count = processors
        .iter()
        .filter(|p| p.Relationship == winnt::RelationProcessorCore)
        .count();

    if count > 0 {
        Ok(Some(count as u64))
    } else {
        Ok(None)
    }
}
