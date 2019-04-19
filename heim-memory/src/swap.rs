use std::fmt;

use heim_common::prelude::*;
use heim_common::units::iec::usize::Information;

use crate::sys;

/// Swap memory statistics.
///
/// Only three metrics are guaranteed to be cross-platform,
/// for other metrics see [SwapExt] trait.
///
/// [SwapExt]: crate::os::SwapExt
#[derive(heim_derive::ImplWrap)]
pub struct Swap(sys::Swap);

impl Swap {
    /// The total amount of swap memory
    pub fn total(&self) -> Information {
        self.0.total()
    }

    /// The used amount of swap memory
    pub fn used(&self) -> Information {
        self.0.used()
    }

    /// The free amount of swap memory
    pub fn free(&self) -> Information {
        self.0.free()
    }
}

impl fmt::Debug for Swap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Swap")
            .field("total", &self.total())
            .field("used", &self.used())
            .field("free", &self.free())
            .finish()
    }
}

/// Returns future which will resolve into [Swap] struct.
pub fn swap() -> impl Future<Item = Swap, Error = Error> {
    sys::swap().map(Into::into)
}
