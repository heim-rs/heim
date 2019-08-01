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

    #[cfg(target_os = "linux")]
    {
        use heim_memory::os::linux::MemoryExt;

        let _ = mem.used();
        let _ = mem.buffers();
        let _ = mem.cached();
        let _ = mem.shared();
        let _ = mem.active();
        let _ = mem.inactive();
    }

    #[cfg(target_os = "macos")]
    {
        use heim_memory::os::macos::MemoryExt;

        let _ = mem.active();
        let _ = mem.inactive();
        let _ = mem.wire();
    }
}

#[runtime::test]
async fn smoke_swap() {
    let swap = memory::swap().await;

    assert!(swap.is_ok());
    let swap = swap.unwrap();

    #[cfg(not(windows))]
    {
        use heim_memory::os::SwapExt;

        let _ = swap.sin();
        let _ = swap.sout();
    }
}
