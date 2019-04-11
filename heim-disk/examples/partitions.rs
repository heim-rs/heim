use heim_runtime::{self as runtime, SyncRuntime};
use heim_common::prelude::*;
use heim_disk as disk;

fn main() -> Result<()> {
    env_logger::init();
    let mut runtime = runtime::new()?;

    for part in runtime.block_collect(disk::partitions()) {
        dbg!(part);
    }

    Ok(())
}
