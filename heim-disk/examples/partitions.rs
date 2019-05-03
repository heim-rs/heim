#![allow(stable_features)]
#![feature(await_macro, async_await, futures_api)]

use heim_common::prelude::*;
use heim_disk as disk;

#[runtime::main]
async fn main() -> Result<()> {
    let mut partitions = disk::partitions();
    while let Some(part) = await!(partitions.next()) {
        dbg!(part?);
    }

    println!("---- Physical partitions ----");

    let mut partitions = disk::partitions_physical();
    while let Some(part) = await!(partitions.next()) {
        dbg!(part?);
    }

    Ok(())
}
