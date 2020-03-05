//! Unix-specific extensions.

use crate::ProcessResult;

mod signal;

pub use self::signal::Signal;

/// Unix-specific extension to [Process].
///
/// [Process]: ../../struct.Process.html
#[async_trait::async_trait]
pub trait ProcessExt {
    /// Send the signal to process.
    async fn signal(&self, signal: Signal) -> ProcessResult<()>;

    /// Get process niceness.
    async fn niceness(&self) -> ProcessResult<libc::c_int>;

    /// Set process niceness.
    async fn set_niceness(&self, value: libc::c_int) -> ProcessResult<()>;
}

#[cfg(unix)]
#[async_trait::async_trait]
impl ProcessExt for crate::Process {
    async fn signal(&self, signal: Signal) -> ProcessResult<()> {
        self.as_ref().signal(signal).await
    }

    async fn niceness(&self) -> ProcessResult<libc::c_int> {
        self.as_ref().niceness().await
    }

    async fn set_niceness(&self, value: libc::c_int) -> ProcessResult<()> {
        self.as_ref().set_niceness(value).await
    }
}
