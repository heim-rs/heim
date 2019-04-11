use heim_common::prelude::*;
use heim_cpu as cpu;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    println!("{:?}", rt.block_run(cpu::time()));

    let cpu_times = rt.block_collect(cpu::times());

    for time in cpu_times {
        println!("{:?}", time);
    }

    Ok(())
}
