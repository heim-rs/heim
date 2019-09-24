use heim_common::prelude::*;
use heim_common::units::{time, Time};

pub fn uptime() -> impl Future<Output = Result<Time>> {
    let ms = unsafe { winapi::um::sysinfoapi::GetTickCount64() } as f64; // TODO: Possible value truncation

    future::ok(Time::new::<time::millisecond>(ms))
}
