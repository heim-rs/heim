mod blocking;
pub(crate) mod fs;

pub mod task {
    use super::blocking;

    #[inline]
    pub async fn spawn_blocking<F, T>(f: F) -> Result<T, crate::task::JoinError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        Ok(blocking::spawn(f).await)
    }
}
