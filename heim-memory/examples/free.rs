use heim_common::prelude::*;
use heim_common::units::iec::information::megabyte;
use heim_runtime::{self as runtime, SyncRuntime};

use heim_memory as memory;

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    let memory = rt.block_run(memory::memory())?;
    let swap = rt.block_run(memory::swap())?;

    println!("              total        free   available");
    println!(
        "{:>7} {:>11?} {:>11?} {:>11?}",
        "Mem:",
        memory.total().get::<megabyte>(),
        memory.free().get::<megabyte>(),
        memory.available().get::<megabyte>(),
    );
    println!(
        "{:>7} {:>11?} {:>11?} {:>11?}",
        "Swap:",
        swap.total().get::<megabyte>(),
        swap.used().get::<megabyte>(),
        swap.free().get::<megabyte>(),
    );

    Ok(())
}
