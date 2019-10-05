use heim_common::prelude::*;

use crate::{sys, Time};

/// Returns future which resolves into [Time] amount from the system boot.
///
/// [Time]: ./struct.Time.html
pub async fn uptime() -> Result2<Time> {
    sys::uptime().await
}
