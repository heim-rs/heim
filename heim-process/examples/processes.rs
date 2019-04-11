use heim_common::prelude::*;
use heim_process as process;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut rt = runtime::new()?;

    for process in rt.block_collect(process::processes()) {
        println!("{:?}", process);
    }

    Ok(())
}
