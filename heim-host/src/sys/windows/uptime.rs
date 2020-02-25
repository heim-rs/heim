use heim_common::prelude::*;
use heim_common::units::{time, Time};

pub async fn uptime() -> Result<Time> {
    let ms = unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64
        // seems not to be able to fail
        winapi::um::sysinfoapi::GetTickCount64()
    } as f64; // TODO: Possible value truncation

    Ok(Time::new::<time::millisecond>(ms))
}
