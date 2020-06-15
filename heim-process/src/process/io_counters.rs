use heim_common::prelude::wrap;
use std::fmt;

use crate::sys;

/// IO information about the process.
///
/// See os-specific extensions also.
pub struct IoCounters(sys::IoCounters);

wrap!(IoCounters, sys::IoCounters);

impl fmt::Debug for IoCounters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IoCounters").finish()
    }
}
