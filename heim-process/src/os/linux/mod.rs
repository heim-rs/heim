//! Linux-specific extensions.

use heim_common::prelude::BoxStream;

use crate::ProcessResult;

mod io_counters;
mod memory;

pub use self::io_counters::IoCountersExt;
pub use self::memory::MemoryExt;

/// Linux-specific extension to [Process]
///
/// [Process]: ../../struct.Process.html
#[async_trait::async_trait]
pub trait ProcessExt {
    /// Returns stream which yield this process [IO counters] for each network interface.
    ///
    /// Since `-> impl Trait` is not allowed yet in the trait methods,
    /// this method returns boxed `Stream`. This behavior will change later.
    ///
    /// [IO counters]: ./struct.IoCounters.html
    #[cfg(target_os = "linux")] // TODO: will be undocumented for other platforms
    async fn net_io_counters(
        &self,
    ) -> ProcessResult<BoxStream<'_, ProcessResult<heim_net::IoCounters>>>;
}

#[cfg(target_os = "linux")]
#[async_trait::async_trait]
impl ProcessExt for crate::Process {
    async fn net_io_counters(
        &self,
    ) -> ProcessResult<BoxStream<'_, ProcessResult<heim_net::IoCounters>>> {
        let stream = self.as_ref().net_io_counters().await?;

        Ok(stream)
    }
}
