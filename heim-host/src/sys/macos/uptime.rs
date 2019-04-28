use heim_common::prelude::*;
use heim_common::units::si::f64::Time;
use heim_common::units::si::time::nanosecond;

use super::TIME_BASE;

pub fn uptime() -> impl Future<Output=Result<Time>> {
    // TODO: Possible value truncation
    let abs_time = unsafe { libc::mach_absolute_time() } as f64;

    let nano_secs = abs_time * *TIME_BASE;

    future::ok(Time::new::<nanosecond>(nano_secs))
}
