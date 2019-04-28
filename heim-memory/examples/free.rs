#![feature(await_macro, async_await)]

use heim_common::prelude::*;
use heim_common::units::iec::information::megabyte;

use heim_memory as memory;

#[runtime::main]
async fn main() -> Result<()> {
    let memory = await!(memory::memory())?;
    let swap = await!(memory::swap())?;

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
