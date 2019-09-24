use std::mem;

use winapi::shared::minwindef;
use winapi::um::sysinfoapi;

use heim_common::prelude::*;
use heim_common::sys::IntoTime;
use heim_common::units::{time, Time};

pub fn boot_time() -> impl Future<Output = Result<Time>> {
    let mut filetime = mem::MaybeUninit::<minwindef::FILETIME>::uninit();

    // `time` value is now a time amount from the January 1, 1601
    let time = unsafe {
        sysinfoapi::GetSystemTimeAsFileTime(filetime.as_mut_ptr());
        filetime.assume_init().into_time()
    };

    /// Seconds amount between the "Windows epoch" (January 1, 1601)
    /// and the Unix epoch (January 1, 1970).
    // TODO: It would be nice to make it const,
    // as soon as `uom` will mark `Time::new` as a `const fn`
    let unix_epoch_delta = Time::new::<time::second>(11_644_473_600.0);

    future::ok(time - unix_epoch_delta)
}
