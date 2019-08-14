#![feature(async_await)]

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

    Ok(())
}
