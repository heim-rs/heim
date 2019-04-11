use heim_common::prelude::*;
use heim_process as process;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut rt = runtime::new()?;

    println!("USER\tPID\t%MEM\tVSZ\tRSS\tNICE\tSTATUS\tSTART TIME\tCMDLINE");
    for process in rt.block_collect(process::processes()) {
        let process = process?;

        println!(" \t{}", process.pid(),);
    }

    Ok(())
}
