use heim_common::prelude::*;
use heim_common::units::si::f64::Time;
use heim_common::units::si::time::millisecond;

pub fn uptime() -> impl Future<Item=Time, Error=Error> {
    let ms = unsafe {
        winapi::um::sysinfoapi::GetTickCount64()
    } as f64; // TODO: Possible value truncation

    future::ok(Time::new::<millisecond>(ms))
}
