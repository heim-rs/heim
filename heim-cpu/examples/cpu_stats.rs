use heim_runtime::{self as runtime, SyncRuntime};
use heim_common::prelude::*;
use heim_cpu as cpu;

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    let stats = rt.block_run(cpu::stats());

    dbg!(stats);

    Ok(())
}
