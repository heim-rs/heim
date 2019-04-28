#![feature(await_macro, async_await, futures_api)]

use heim_common::units::iec::information::byte;
use heim_memory as memory;

#[runtime::test]
async fn smoke_memory() {
    let mem = await!(memory::memory());

    assert!(mem.is_ok());
    let mem = mem.unwrap();

    assert!(mem.total().get::<byte>() > 0);
    assert!(mem.available().get::<byte>() > 0);
    assert!(mem.free().get::<byte>() > 0);
}

#[runtime::test]
async fn smoke_swap() {
    let swap = await!(memory::swap());

    assert!(swap.is_ok());
}
