use std::fmt;

use crate::{sys, units};
use heim_common::prelude::*;

/// System CPU time.
#[derive(heim_derive::ImplWrap)]
pub struct CpuTime(sys::CpuTime);

impl CpuTime {
    /// Returns time spent by normal processes executing in user mode.
    ///
    /// ## Compatibility
    ///
    ///  * on Linux this also includes guest time
    pub fn user(&self) -> units::Time {
        self.as_ref().user()
    }

    /// Returns time spent by processes executing in kernel mode.
    pub fn system(&self) -> units::Time {
        self.as_ref().system()
    }

    /// Returns time spent doing nothing.
    pub fn idle(&self) -> units::Time {
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
pub fn time() -> impl Future<Item = CpuTime, Error = Error> {
    sys::time().map(Into::into)
}

/// Returns stream which will yield [CPU time] for each CPU.
///
/// [CPU time]: struct.CpuTime.html
pub fn times() -> impl Stream<Item = CpuTime, Error = Error> {
    sys::times().map(Into::into)
}
