use heim_common::prelude::*;
use heim_runtime::{self as runtime, SyncRuntime};
use heim_process as process;

fn main() -> Result<()> {
    let mut rt = runtime::new()?;

    for process in rt.block_collect(process::processes()) {
        dbg!(process);
    }

    Ok(())
}
