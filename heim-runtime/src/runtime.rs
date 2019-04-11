use tokio::prelude::*;
use tokio::runtime::{self, current_thread};
use tokio::sync::mpsc;

/// Extension for `tokio` runtime, which runs `Future`s and `Stream`s in a blocking manner.
pub trait SyncRuntime {
    /// Blocks current thread and drives passed future to the completion.
    fn block_run<F>(&mut self, f: F) -> Result<F::Item, F::Error>
    where
        F: Future + Send + 'static,
        F::Item: Send + 'static,
        F::Error: Send + 'static;

    /// Returns iterator which will yield values from the passed stream.
    fn block_collect<S>(&mut self, stream: S) -> Collect<Result<S::Item, S::Error>>
    where
        S: Stream + Send + 'static,
        S::Item: Send + 'static,
        S::Error: Send + 'static;
}

impl SyncRuntime for current_thread::Runtime {
    fn block_run<F>(&mut self, f: F) -> Result<<F as Future>::Item, <F as Future>::Error>
    where
        F: Future + Send + 'static,
        F::Item: Send + 'static,
        F::Error: Send + 'static,
    {
        self.block_on(f)
    }

    fn block_collect<S>(&mut self, stream: S) -> Collect<Result<<S as Stream>::Item, <S as Stream>::Error>>
    where
        S: Stream + Send + 'static,
        S::Item: Send + 'static,
        S::Error: Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel::<Result<S::Item, S::Error>>();
        let runner = future::lazy(move || {
            stream
                .then(move |res| tx.clone().send(res))
                .for_each(|_| Ok(()))
                .map_err(|_| ())
        });

        self.spawn(runner);

        Collect {
            rx: Some(rx.into_future()),
        }
    }
}

impl SyncRuntime for runtime::Runtime {
    fn block_run<F>(&mut self, f: F) -> Result<<F as Future>::Item, <F as Future>::Error>
    where
        F: Future + Send + 'static,
        F::Item: Send + 'static,
        F::Error: Send + 'static,
    {
        self.block_on(f)
    }

    fn block_collect<S>(&mut self, stream: S) -> Collect<Result<<S as Stream>::Item, <S as Stream>::Error>>
    where
        S: Stream + Send + 'static,
        S::Item: Send + 'static,
        S::Error: Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel::<Result<S::Item, S::Error>>();
        let runner = future::lazy(move || {
            stream
                .then(move |res| tx.clone().send(res))
                .for_each(|_| Ok(()))
                .map_err(|_| ())
        });

        self.spawn(runner);

        Collect {
            rx: Some(rx.into_future()),
        }
    }
}

#[derive(Debug)]
pub struct Collect<T> {
    rx: Option<stream::StreamFuture<mpsc::UnboundedReceiver<T>>>,
}

impl<T> Iterator for Collect<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.take().and_then(|rx| {
            match rx.wait() {
                // Ended stream will return `item = None` here,
                // which will terminate the iteration
                Ok((item, new_rx)) => {
                    let _ = self.rx.replace(new_rx.into_future());
                    item
                }
                Err(_e) => None,
            }
        })
    }
}
