use std::io;
use std::mem;

use mach::{kern_return, mach_time};

mod uptime;

pub use self::uptime::*;

unsafe fn timebase_info() -> io::Result<mach_time::mach_timebase_info> {
    // TODO: Use MaybeUninit here
    let mut info: mach_time::mach_timebase_info = mem::zeroed();
    let res = mach_time::mach_timebase_info(&mut info);

    if res == kern_return::KERN_SUCCESS {
        Ok(info)
    } else {
        Err(io::Error::last_os_error())
    }
}


lazy_static::lazy_static! {
    // Calling `mach_timebase_info` is not cheap, so we should cache it.
    // https://github.com/joyent/libuv/pull/1325
    pub static ref TIME_BASE: f64 = {
        // It is nearly impossible to get the panic here
        let info = unsafe { timebase_info().expect("Unable to get mach timebase info") };
        // We are going to use this fields as a `f64` types later in the `uptime` function,
        // so why can't we convert them only once?
        (f64::from(info.numer) / f64::from(info.denom))
    };
}
