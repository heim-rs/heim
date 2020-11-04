//! Naive clone of the `free` utility

use std::error::Error;

use heim::{memory, units::information};

fn main() -> Result<(), Box<dyn Error>> {
    smol::block_on(async {
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
    })
}
