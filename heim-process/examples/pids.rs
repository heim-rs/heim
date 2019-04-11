use heim_common::prelude::*;
use heim_process as process;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut rt = runtime::new()?;

    for pid in rt.block_collect(process::pids()) {
        println!("{:?}", pid);
    }

    Ok(())
}
