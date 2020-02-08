use std::fmt;

use heim_common::prelude::*;
use heim_common::units::Time;

use crate::sys;

/// System CPU time.
///
/// ## Compatibility
///
/// For Linux additional information can be retrieved with [CpuTimeExt] extension trait.
///
/// [CpuTimeExt]: ./os/linux/trait.CpuTimeExt.html
pub struct CpuTime(sys::CpuTime);

wrap!(CpuTime, sys::CpuTime);

impl CpuTime {
    /// Returns time spent by normal processes executing in user mode.
    ///
    /// ## Compatibility
    ///
    ///  * on Linux this also includes guest time
    pub fn user(&self) -> Time {
        self.as_ref().user()
    }

    /// Returns time spent by processes executing in kernel mode.
    pub fn system(&self) -> Time {
        self.as_ref().system()
    }

    /// Returns time spent doing nothing.
    pub fn idle(&self) -> Time {
        self.as_ref().idle()
    }
}

impl fmt::Debug for CpuTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CpuTime")
            .field("user", &self.user())
            .field("system", &self.system())
            .field("idle", &self.idle())
            .finish()
    }
}

/// Returns future which will resolve into cumulative value of all [CPU times].
///
/// [CPU times]: struct.CpuTime.html
pub fn time() -> impl Future<Output = Result<CpuTime>> {
    sys::time().map_ok(Into::into)
}

/// Returns stream which will yield [CPU time] for each CPU.
///
/// [CPU time]: struct.CpuTime.html
pub fn times() -> impl Stream<Item = Result<CpuTime>> {
    sys::times().map_ok(Into::into)
}
