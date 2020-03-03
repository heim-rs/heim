//! Linux-specific extensions.

use heim_common::units::Ratio;
use heim_common::Result;

use crate::sys;

/// Returns the average system load over the last 1, 5 and 15 minutes.
///
/// The load represents the processes which are in a runnable state,
/// either using the CPU or waiting to use the CPU (e.g. waiting for disk I/O).
pub async fn loadavg() -> Result<(Ratio, Ratio, Ratio)> {
    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            sys::loadavg().await
        } else {
            unimplemented!("For documentation rendering")
        }
    }
}
