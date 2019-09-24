//! Linux-specific extensions.

use heim_common::prelude::{BoxFuture, BoxStream};

use crate::ProcessResult;

mod io_counters;
mod memory;

pub use self::io_counters::IoCounters;
pub use self::memory::MemoryExt;

/// Linux-specific extension to [Process]
///
/// [Process]: ../../struct.Process.html
#[heim_derive::os_ext_for(crate::Process, cfg(target_os = "linux"))]
pub trait ProcessExt {
    /// Returns future which resolves into process IO counters.
    ///
    /// Since `-> impl Trait` is not allowed yet in the trait methods,
    /// this method returns boxed `Future`. This behavior will change later.
    fn io_counters(&self) -> BoxFuture<ProcessResult<IoCounters>>;

    /// Returns stream which yield this process [IO counters] for each network interface.
    ///
    /// Since `-> impl Trait` is not allowed yet in the trait methods,
    /// this method returns boxed `Stream`. This behavior will change later.
    ///
    /// [IO counters]: ./struct.IoCounters.html
    fn net_io_counters(&self) -> BoxStream<ProcessResult<heim_net::IoCounters>>;
}
