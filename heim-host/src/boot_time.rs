use heim_common::prelude::*;

use crate::{sys, Time};

/// Returns system boot [Time] since the UNIX epoch.
///
/// [Time]: ./struct.Time.html
pub async fn boot_time() -> Result<Time> {
    sys::boot_time().await
}
