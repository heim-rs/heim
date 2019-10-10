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
    ///
    /// Since `-> impl Trait` is not allowed yet in the trait methods,
    /// this method returns boxed `Future`. This behavior will change later.
    async fn signal(&self, signal: Signal) -> ProcessResult<()>;
}

#[cfg(unix)]
#[async_trait::async_trait]
impl ProcessExt for crate::Process {
    async fn signal(&self, signal: Signal) -> ProcessResult<()> {
        self.as_ref().signal(signal).await
    }
}
