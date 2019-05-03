#![allow(stable_features)]
#![feature(await_macro, async_await, futures_api)]

use heim_memory as memory;

#[runtime::test]
async fn smoke_memory() {
    let mem = await!(memory::memory());

    assert!(mem.is_ok());
    let mem = mem.unwrap();

    assert!(mem.total().get() > 0);
    assert!(mem.available().get() > 0);
    assert!(mem.free().get() > 0);
}

#[runtime::test]
async fn smoke_swap() {
    let swap = await!(memory::swap());

    assert!(swap.is_ok());
}
