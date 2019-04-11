use heim_common::prelude::*;
use heim_runtime::{self as runtime, SyncRuntime};
use heim_cpu as cpu;

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    dbg!(rt.block_run(cpu::frequency()));

    #[cfg(target_os = "linux")]
    dbg!(rt.block_collect(cpu::os::linux::frequencies()).collect::<Vec<_>>());

    Ok(())
}
