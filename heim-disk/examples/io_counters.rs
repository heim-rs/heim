use heim_common::prelude::*;
use heim_disk as disk;

#[heim_derive::main]
async fn main() -> Result2<()> {
    let counters = disk::io_counters();
    pin_utils::pin_mut!(counters);
    while let Some(counter) = counters.next().await {
        dbg!(counter?);
    }

    println!("\n\n--- Per physical disk ---\n");

    let counters = disk::io_counters_physical();
    pin_utils::pin_mut!(counters);
    while let Some(counter) = counters.next().await {
        dbg!(counter?);
    }

    Ok(())
}
