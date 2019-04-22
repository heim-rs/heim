use heim_common::prelude::*;
use heim_disk as disk;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    env_logger::init();
    let mut runtime = runtime::new()?;

    for part in runtime.block_collect(disk::partitions()) {
        println!("{:?}", part);
    }

    println!("---- Physical partitions ----");

    for part in runtime.block_collect(disk::partitions_physical()) {
        println!("{:?}", part);
    }

    Ok(())
}
