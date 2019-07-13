use heim_common::prelude::{Future, Result};

use crate::sys;

/// Returns future which will resolve into a amount of logical CPUs.
pub fn logical_count() -> impl Future<Output = Result<u64>> {
    sys::logical_count()
}

/// Returns future which will resolve into a amount of physical CPUs.
///
/// Returned future might resolve into `Ok(None)` if the amount can't be determined.
pub fn physical_count() -> impl Future<Output = Result<Option<u64>>> {
    sys::physical_count()
}
