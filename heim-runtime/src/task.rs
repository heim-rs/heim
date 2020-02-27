use std::error::Error;
use std::fmt;
use std::io;

/// Spawn blocking task on the async runtime
#[inline]
pub async fn spawn_blocking<F, T>(f: F) -> Result<T, JoinError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    crate::runtime::task::spawn_blocking(f).await
}

#[derive(Debug)]
pub enum JoinError {
    /// Spawned task was cancelled.
    Cancel,
    /// Spawned task had panicked.
    Panic,
    /// Fallback option
    Other,
}

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JoinError::Cancel => f.write_str("Blocking task was cancelled"),
            JoinError::Panic => f.write_str("Blocking task had panicked"),
            JoinError::Other => f.write_str("Failed to join blocking task"),
        }
    }
}

impl Error for JoinError {}

impl From<JoinError> for io::Error {
    fn from(e: JoinError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, e)
    }
}
