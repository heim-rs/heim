use heim_common::prelude::*;


pub fn logical_count() -> impl Future<Output = Result<u64>> {
    // TODO: Stub, see https://github.com/heim-rs/heim/issues/50
    future::ok(1)
}

pub fn physical_count() -> impl Future<Output = Result<Option<u64>>> {
    // TODO: Stub, see https://github.com/heim-rs/heim/issues/53
    future::ok(None)
}
