use heim_common::prelude::*;
use heim_runtime::{self as runtime, SyncRuntime};
use heim_cpu as cpu;

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    dbg!(rt.block_run(cpu::time()));

    let cpu_times = rt.block_collect(cpu::times());

    for time in cpu_times {
        dbg!(time);
    }

    Ok(())
}
