use heim_common::prelude::*;

use crate::{sys, Time};

/// Returns future which resolves into [Time] amount from the system boot.
///
/// [Time]: ./struct.Time.html
pub fn uptime() -> impl Future<Output = Result<Time>> {
    sys::uptime()
}
