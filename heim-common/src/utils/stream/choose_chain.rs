use core::pin::Pin;
use pin_utils::{unsafe_pinned, unsafe_unpinned};

use crate::prelude::{
    futures,
    futures::task::{Context, Poll},
    FusedStream, Stream,
};

/// Stream for the [`choose_chain`](super::HeimStreamExt::choose_chain) method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct ChooseChain<St1, St2> {
    first: Option<St1>,
    first_yielded: bool,
    second: St2,
}

impl<St1, St2> ChooseChain<St1, St2>
where
    St1: Stream,
    St2: Stream<Item = St1::Item>,
{
    unsafe_pinned!(first: Option<St1>);
    unsafe_unpinned!(first_yielded: bool);
    unsafe_pinned!(second: St2);

    pub(super) fn new(stream1: St1, stream2: St2) -> ChooseChain<St1, St2> {
        ChooseChain {
            first: Some(stream1),
            first_yielded: false,
            second: stream2,
        }
    }
}

impl<St1: Stream, St2: FusedStream<Item = St1::Item>> FusedStream for ChooseChain<St1, St2> {
    fn is_terminated(&self) -> bool {
        self.first.is_none() && self.second.is_terminated()
    }
}

impl<St1, St2> Stream for ChooseChain<St1, St2>
where
    St1: Stream,
    St2: Stream<Item = St1::Item>,
{
    type Item = St1::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(first) = self.as_mut().first().as_pin_mut() {
            if let Some(item) = futures::ready!(first.poll_next(cx)) {
                *self.as_mut().first_yielded() = true;
                return Poll::Ready(Some(item));
            }
        }
        self.as_mut().first().set(None);
        if self.first_yielded {
            Poll::Ready(None)
        } else {
            self.as_mut().second().poll_next(cx)
        }
    }
}

#[cfg(test)]
mod tests {
    use futures_executor as executor;

    use crate::prelude::stream;

    use super::super::HeimStreamExt;

    #[test]
    fn both_empty() {
        let s1 = stream::iter(Vec::<i32>::new());
        let s2 = stream::iter(Vec::<i32>::new());

        let chain = s1.choose_chain(s2);
        let results = executor::block_on_stream(chain).collect::<Vec<_>>();

        assert!(results.is_empty());
    }

    #[test]
    fn first_empty() {
        let s1 = stream::iter(Vec::<i32>::new());
        let s2 = stream::iter(vec![4, 5, 6]);

        let chain = s1.choose_chain(s2);
        let results = executor::block_on_stream(chain).collect::<Vec<_>>();

        assert_eq!(results, vec![4, 5, 6]);
    }

    #[test]
    fn second_empty() {
        let s1 = stream::iter(vec![1, 2, 3]);
        let s2 = stream::iter(Vec::<i32>::new());

        let chain = s1.choose_chain(s2);
        let results = executor::block_on_stream(chain).collect::<Vec<_>>();

        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn both_filled() {
        let s1 = stream::iter(vec![1, 2, 3]);
        let s2 = stream::iter(vec![4, 5, 6]);

        let chain = s1.choose_chain(s2);
        let results = executor::block_on_stream(chain).collect::<Vec<_>>();

        assert_eq!(results, vec![1, 2, 3]);
    }
}
