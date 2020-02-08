//! Unix-specific extensions.

use heim_common::prelude::BoxFuture;

use crate::ProcessResult;

mod signal;

pub use self::signal::Signal;

/// Unix-specific extension to [Process].
///
/// [Process]: ../../struct.Process.html
pub trait ProcessExt {
    /// Send the signal to process.
    ///
    /// Since `-> impl Trait` is not allowed yet in the trait methods,
    /// this method returns boxed `Future`. This behavior will change later.
    fn signal(&self, signal: Signal) -> BoxFuture<ProcessResult<()>>;
}

#[cfg(unix)]
impl ProcessExt for crate::Process {
    fn signal(&self, signal: Signal) -> BoxFuture<ProcessResult<()>> {
        self.as_ref().signal(signal)
    }
}
