//! Backported `futures::stream::select_all` for futures 0.1
//!
//! https://github.com/rust-lang-nursery/futures-rs/blob/465ef06d104e2f15098e30971c12642a55a6501b/futures-util/src/stream/select_all.rs

use std::default::Default;

use tokio::prelude::*;

#[must_use = "streams do nothing unless polled"]
pub struct SelectAll<S> {
    inner: stream::FuturesUnordered<stream::StreamFuture<S>>,
}

impl<S> SelectAll<S>
where
    S: Stream,
{
    pub fn new() -> SelectAll<S> {
        SelectAll {
            inner: stream::FuturesUnordered::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, stream: S) {
        self.inner.push(stream.into_future());
    }
}

impl<S> Default for SelectAll<S>
where
    S: Stream,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Stream for SelectAll<S>
where
    S: Stream,
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.inner.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(Some((Some(item), remaining)))) => {
                self.push(remaining);
                Ok(Async::Ready(Some(item)))
            }
            Ok(Async::Ready(Some((None, _)))) => {
                let _ = self.inner.poll();
                Ok(Async::NotReady)
            }
            Ok(Async::Ready(_)) => Ok(Async::Ready(None)),
            Err((e, _)) => Err(e),
        }
    }
}

pub fn select_all<I, T, E>(streams: I) -> SelectAll<impl Stream<Item = T, Error = E> + Send>
where
    I: IntoIterator,
    I::Item: Stream<Item = T, Error = E> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    let mut select = SelectAll::new();
    for stream in streams {
        select.push(stream);
    }

    select
}
