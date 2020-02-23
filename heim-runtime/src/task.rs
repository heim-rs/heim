use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct JoinError;

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Failed blocking task")
    }
}

impl Error for JoinError {}

/// Spawn blocking task on the async runtime
#[inline]
pub async fn spawn_blocking<F, T>(f: F) -> Result<T, JoinError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    crate::runtime::task::spawn_blocking(f).await
}
