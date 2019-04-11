use heim_common::prelude::*;
use heim_cpu as cpu;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    let stats = rt.block_run(cpu::stats());

    println!("{:?}", stats);

    Ok(())
}
