use heim_common::prelude::*;
use heim_common::units::information;
use heim_memory as memory;

#[heim_derive::main]
async fn main() -> Result<()> {
    let memory = memory::memory().await?;
    let swap = memory::swap().await?;

    println!("              total        free   available");
    println!(
        "{:>7} {:>11?} {:>11?} {:>11?}",
        "Mem:",
        memory.total().get::<information::megabyte>(),
        memory.free().get::<information::megabyte>(),
        memory.available().get::<information::megabyte>(),
    );
    println!(
        "{:>7} {:>11?} {:>11?} {:>11?}",
        "Swap:",
        swap.total().get::<information::megabyte>(),
        swap.used().get::<information::megabyte>(),
        swap.free().get::<information::megabyte>(),
    );

    Ok(())
}
