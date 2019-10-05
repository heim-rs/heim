use heim_common::prelude::Result2;

use crate::sys;

/// Returns future which will resolve into a amount of logical CPUs.
pub async fn logical_count() -> Result2<u64> {
    sys::logical_count().await
}

/// Returns future which will resolve into a amount of physical CPUs.
///
/// Returned future might resolve into `Ok(None)` if the amount can't be determined.
pub async fn physical_count() -> Result2<Option<u64>> {
    sys::physical_count().await
}
