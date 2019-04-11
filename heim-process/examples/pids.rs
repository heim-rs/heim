use heim_common::prelude::*;
use heim_runtime::{self as runtime, SyncRuntime};
use heim_process as process;

fn main() -> Result<()> {
    let mut rt = runtime::new()?;

    for pid in rt.block_collect(process::pids()) {
        dbg!(pid);
    }

    Ok(())
}
