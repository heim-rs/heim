use heim_common::prelude::*;

use crate::{sys, Time};

/// Returns [Time] amount from the system boot.
///
/// [Time]: ./struct.Time.html
pub async fn uptime() -> Result<Time> {
    sys::uptime().await
}
