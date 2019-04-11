use heim_runtime::{self as runtime, SyncRuntime};
use heim_process as process;

#[test]
fn smoke_pids() {
    let mut rt = runtime::new().unwrap();
    let pids = rt.block_collect(process::pids());

    assert_ne!(0, pids.count());
}

#[test]
fn smoke_processes() {
    let mut rt = runtime::new().unwrap();
    let processes = rt.block_collect(process::processes());

    assert_ne!(0, processes.count());
}

