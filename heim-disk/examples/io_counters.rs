#![feature(async_await)]

use heim_common::prelude::*;
use heim_disk as disk;

#[runtime::main]
async fn main() -> Result<()> {
    let mut counters = disk::io_counters();
    while let Some(counter) = counters.next().await {
        dbg!(counter?);
    }

    println!("\n\n--- Per physical disk ---\n");

    let mut counters = disk::io_counters_physical();
    while let Some(counter) = counters.next().await {
        dbg!(counter?);
    }

    Ok(())
}
