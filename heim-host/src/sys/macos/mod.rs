use std::mem;

use heim_common::{Error, Result};
use mach::{kern_return, mach_time};

mod boot_time;
mod uptime;
mod users;

pub use self::boot_time::*;
pub use self::uptime::*;
pub use self::users::*;

fn timebase_info() -> Result<mach_time::mach_timebase_info> {
    let mut info = mem::MaybeUninit::<mach_time::mach_timebase_info>::uninit();
    let res = unsafe { mach_time::mach_timebase_info(info.as_mut_ptr()) };

    if res == kern_return::KERN_SUCCESS {
        Ok(unsafe { info.assume_init() })
    } else {
        Err(Error::last_os_error().with_ffi("mach_timebase_info"))
    }
}

lazy_static::lazy_static! {
    // Calling `mach_timebase_info` is not cheap, so we should cache it.
    // https://github.com/joyent/libuv/pull/1325
    pub static ref TIME_BASE: f64 = {
        // It is nearly impossible to get the panic here
        let info = timebase_info().expect("Unable to get mach timebase info");
        // We are going to use its fields as a `f64` types later in the `uptime` function,
        // so why can't we convert them only once?
        f64::from(info.numer) / f64::from(info.denom)
    };
}
