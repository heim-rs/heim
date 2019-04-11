use heim_common::units::iec::information::byte;
use heim_memory as memory;
use heim_runtime::{self as runtime, SyncRuntime};

#[test]
fn smoke_memory() {
    let mut rt = runtime::new().unwrap();
    let mem = rt.block_run(memory::memory());

    assert!(mem.is_ok());
    let mem = mem.unwrap();

    assert!(mem.total().get::<byte>() > 0);
    assert!(mem.available().get::<byte>() > 0);
    assert!(mem.free().get::<byte>() > 0);
}

#[test]
fn smoke_swap() {
    let mut rt = runtime::new().unwrap();
    let swap = rt.block_run(memory::swap());

    assert!(swap.is_ok());
}
