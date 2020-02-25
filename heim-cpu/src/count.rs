use heim_common::prelude::Result;

use crate::sys;

/// Returns an amount of logical CPUs.
pub async fn logical_count() -> Result<u64> {
    sys::logical_count().await
}

/// Returns an amount of physical CPUs.
///
/// ## Returns
///
/// If the amount can't be determined, `Ok(None)` will be returned.
pub async fn physical_count() -> Result<Option<u64>> {
    sys::physical_count().await
}
