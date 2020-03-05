//! Windows-specific extensions.

use crate::ProcessResult;

mod memory;
mod priority;

pub use self::memory::MemoryExt;
pub use self::priority::Priority;

/// Windows-specific extension to [Process].
///
/// [Process]: ../../struct.Process.html
#[async_trait::async_trait]
pub trait ProcessExt {
    /// Get process priority.
    async fn priority(&self) -> ProcessResult<Priority>;

    /// Set process priority.
    async fn set_priority(&self, value: Priority) -> ProcessResult<()>;
}

#[cfg(target_os = "windows")]
#[async_trait::async_trait]
impl ProcessExt for crate::Process {
    async fn priority(&self) -> ProcessResult<Priority> {
        self.as_ref().priority().await
    }

    async fn set_priority(&self, value: Priority) -> ProcessResult<()> {
        self.as_ref().set_priority(value).await
    }
}
