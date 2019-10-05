use heim_common::prelude::*;
use heim_common::units::{time, Time};

pub async fn uptime() -> Result2<Time> {
    let ms = unsafe { winapi::um::sysinfoapi::GetTickCount64() } as f64; // TODO: Possible value truncation

    Ok(Time::new::<time::millisecond>(ms))
}
