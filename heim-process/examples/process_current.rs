#![feature(async_await)]

use futures::stream::StreamExt;
use heim_process as process;

#[heim_derive::main]
async fn main() -> Result<(), process::ProcessError> {
    let process = process::Process::current().await?;

    dbg!(process.pid());
    dbg!(process.parent_pid().await?);
    dbg!(process.status().await?);
    dbg!(process.name().await?);
    dbg!(process.exe().await?);
    dbg!(process.cpu_time().await?);
    dbg!(process.memory().await?);

    let mut net_io_counters = process.net_io_counters();
    while let Some(counter) = net_io_counters.next().await {
        let counter = counter?;
        dbg!(counter);
    }

    Ok(())
}
