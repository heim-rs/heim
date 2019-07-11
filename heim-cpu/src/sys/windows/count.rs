use winapi::um::{winbase, winnt};

use heim_common::prelude::*;


pub fn logical_count() -> impl Future<Output = Result<u64>> {
    // TODO: Stub, see https://github.com/heim-rs/heim/issues/51
    let result = unsafe {
        winbase::GetActiveProcessorCount(winnt::ALL_PROCESSOR_GROUPS)
    };

    if result > 0 {
        future::ok(u64::from(result))
    } else {
        future::err(Error::last_os_error())
    }
}

pub fn physical_count() -> impl Future<Output = Result<Option<u64>>> {
    // TODO: Stub, see https://github.com/heim-rs/heim/issues/54
    future::ok(None)
}
