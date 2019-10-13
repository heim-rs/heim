use heim_common::prelude::*;
use heim_disk as disk;

#[heim_derive::main]
async fn main() -> Result<()> {
    println!("---- Partitions ----");
    let partitions = disk::partitions();
    pin_utils::pin_mut!(partitions);
    while let Some(part) = partitions.next().await {
        dbg!(part?);
    }

    println!("---- Physical partitions ----");

    let partitions = disk::partitions_physical();
    pin_utils::pin_mut!(partitions);
    while let Some(part) = partitions.next().await {
        dbg!(part?);
    }

    Ok(())
}
