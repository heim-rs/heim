#![feature(async_await)]

use heim_memory as memory;

#[runtime::test]
async fn smoke_memory() {
    let mem = memory::memory().await;

    assert!(mem.is_ok());
    let mem = mem.unwrap();

    assert!(mem.total().get() > 0);
    assert!(mem.available().get() > 0);
    assert!(mem.free().get() > 0);
}

#[runtime::test]
async fn smoke_swap() {
    let swap = memory::swap().await;

    assert!(swap.is_ok());
}
