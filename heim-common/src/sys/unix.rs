//! Unix-specific routines used across `heim` crates.

use crate::{Error, Result};

fn clock_ticks() -> Result<u64> {
    let result = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };

    if result > 0 {
        Ok(result as u64)
    } else {
        Err(Error::last_os_error().with_sysconf(libc::_SC_CLK_TCK))
    }
}

lazy_static::lazy_static! {
    /// Time units in USER_HZ or Jiffies
    pub static ref CLOCK_TICKS: u64 = clock_ticks()
        .expect("Unable to determine CPU number of ticks per second");
}
