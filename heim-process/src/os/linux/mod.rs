//! Linux-specific extensions.

use futures::future::BoxFuture;

use crate::ProcessResult;

mod memory;
mod io_counters;

pub use self::memory::MemoryExt;
pub use self::io_counters::IoCounters;

/// Linux-specific extension to [Process]
///
/// [Process]: ../../struct.Process.html
#[heim_derive::os_ext_for(crate::Process, cfg(target_os = "linux"))]
pub trait ProcessExt {
    /// Returns future which resolves into process IO counters.
    ///
    /// Since `-> impl Trait` is not allowed yet in the traits,
    /// this method returns boxed `Future`. This behavior will change later.
    fn io_counters(&self) -> BoxFuture<ProcessResult<IoCounters>>;
}
