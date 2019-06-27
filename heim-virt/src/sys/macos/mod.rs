use heim_common::prelude::{future, Future};

use crate::Virtualization;

pub fn detect() -> impl Future<Output = Option<Virtualization>> {
    // TODO: stub
    future::ready(None)
}
