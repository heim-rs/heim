use std::fmt;

use heim_common::prelude::*;
use heim_common::units::Information;

use crate::sys;

/// Physical memory statistics.
///
/// Only three metrics are guaranteed to be cross-platform,
/// for other metrics see `MemoryExt` traits in the [os] submodules.
///
/// [os]: ./os/index.html
pub struct Memory(sys::Memory);

wrap!(Memory, sys::Memory);

impl Memory {
    /// The total amount of physical memory.
    pub fn total(&self) -> Information {
        self.as_ref().total()
    }

    /// The amount of memory that can be given instantly to processes
    /// without the system going into swap.
    pub fn available(&self) -> Information {
        self.as_ref().available()
    }

    /// The amount of memory not being used at all (zeroed) that is readily available;
    /// note that this does not reflect the actual memory available.
    pub fn free(&self) -> Information {
        self.as_ref().free()
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory")
            .field("total", &self.total())
            .field("available", &self.available())
            .field("free", &self.free())
            .finish()
    }
}

/// Returns future which will resolve into [Memory] struct.
///
/// [Memory]: ./struct.Memory.html
pub fn memory() -> impl future::Future<Output = Result<Memory>> {
    sys::memory().map(|res| res.map(Into::into))
}
