use std::fmt::Debug;
use std::sync::Mutex;

use futures_channel::oneshot;
use threadpool::ThreadPool;

use heim_common::prelude::*;

lazy_static::lazy_static! {
    pub(crate) static ref THREAD_POOL: FuturePool = FuturePool::new();
}

#[derive(Debug)]
pub struct FuturePool(Mutex<ThreadPool>);

impl FuturePool {
    pub fn new() -> FuturePool {
        FuturePool(Mutex::new(ThreadPool::default()))
    }

    pub fn spawn<F, T>(&self, f: F) -> impl Future<Output = T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + Debug + 'static,
    {
        let pool = self.0.lock().expect("Thread pool mutex is poisoned");

        let (tx, rx) = oneshot::channel();
        pool.execute(move || {
            let _ = tx.send(f());
        });

        rx.map(|res| res.expect("Runtime future was canceled"))
    }
}
