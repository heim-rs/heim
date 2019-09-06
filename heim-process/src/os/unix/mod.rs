//! Unix-specific extensions.

use futures::future::BoxFuture;

use crate::ProcessResult;

mod signal;

pub use self::signal::Signal;

/// Unix-specific extension to [Process].
///
/// [Process]: ../../struct.Process.html
#[heim_derive::os_ext_for(crate::Process, cfg(unix))]
pub trait ProcessExt {
    /// Send the signal to process.
    ///
    /// Since `-> impl Trait` is not allowed yet in the trait methods,
    /// this method returns boxed `Future`. This behavior will change later.
    fn signal(&self, signal: Signal) -> BoxFuture<ProcessResult<()>>;
}
