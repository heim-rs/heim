use heim_common::prelude::*;
use heim_common::units::{time, Time};

use mach::mach_time;

use super::TIME_BASE;

pub async fn uptime() -> Result<Time> {
    // TODO: Possible value truncation
    let abs_time = unsafe { mach_time::mach_absolute_time() } as f64;

    let nano_secs = abs_time * *TIME_BASE;

    Ok(Time::new::<time::nanosecond>(nano_secs))
}
