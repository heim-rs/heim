use std::future::Future;

pub mod fs;
pub mod time;

#[inline]
pub async fn spawn<F, R>(f: F) -> R
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    smol::Task::spawn(f).await
}

pub async fn spawn_blocking<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    smol::Task::blocking(async move { f() }).await
}
