#![feature(async_await)]

use heim_common::prelude::StreamExt;
use heim_process as process;

#[runtime::main]
async fn main() -> Result<(), process::ProcessError> {
    let mut pids = process::pids();
    while let Some(pid) = pids.next().await {
        dbg!(pid?);
    }

    Ok(())
}
