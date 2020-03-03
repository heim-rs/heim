pub mod fs;

pub mod task {
    #[inline]
    pub async fn spawn_blocking<F, T>(f: F) -> Result<T, crate::task::JoinError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        Ok(async_std::task::spawn_blocking(f).await)
    }
}

pub mod time {
    pub use async_std::stream::interval;
}
