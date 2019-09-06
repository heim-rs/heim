//! `futures::Stream` extensions

use crate::prelude::Stream;

mod choose_chain;

pub use self::choose_chain::ChooseChain;

/// heim-specific extensions for `futures::Stream`.
pub trait HeimStreamExt: Stream {
    /// Yields items from the `Self`, and if `Self` yield nothing,
    /// yields from `other` then.
    ///
    /// If `Self` yielded at least one item, `other` will not be polled at all.
    fn choose_chain<St>(self, other: St) -> ChooseChain<Self, St>
    where
        St: Stream<Item = Self::Item>,
        Self: Sized,
    {
        ChooseChain::new(self, other)
    }
}

impl<T> HeimStreamExt for T where T: Stream {}
