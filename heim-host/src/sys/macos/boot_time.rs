use heim_common::prelude::*;

use crate::Time;

pub fn boot_time() -> impl Future<Output = Result<Time>> {
    future::lazy(|_| {
        unimplemented!("https://github.com/heim-rs/heim/issues/148")
    })
}
