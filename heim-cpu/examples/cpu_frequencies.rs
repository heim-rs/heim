use heim_common::prelude::*;
use heim_cpu as cpu;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut rt = runtime::new()?;
    println!("{:#?}", rt.block_run(cpu::frequency()));

    #[cfg(target_os = "linux")]
    println!(
        "{:?}",
        rt.block_collect(cpu::os::linux::frequencies()).collect::<Vec<_>>()
    );

    Ok(())
}
