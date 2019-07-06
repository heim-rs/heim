#![feature(async_await)]

use heim_common::prelude::*;
use heim_memory as memory;

const MEGABYTE: u64 = 1_024 * 1_024;

#[runtime::main]
async fn main() -> Result<()> {
    let memory = memory::memory().await?;
    let swap = memory::swap().await?;

    println!("              total        free   available");
    println!(
        "{:>7} {:>11?} {:>11?} {:>11?}",
        "Mem:",
        memory.total().get() * MEGABYTE,
        memory.free().get() * MEGABYTE,
        memory.available().get() * MEGABYTE,
    );
    println!(
        "{:>7} {:>11?} {:>11?} {:>11?}",
        "Swap:",
        swap.total().get() * MEGABYTE,
        swap.used().get() * MEGABYTE,
        swap.free().get() * MEGABYTE,
    );

    Ok(())
}
