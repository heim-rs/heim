use std::env;

use heim_process as process;

#[heim_derive::main]
async fn main() -> process::ProcessResult<()> {
    let pids = env::args().filter_map(|arg| arg.parse::<process::Pid>().ok());

    for pid in pids {
        let process = process::get(pid).await?;
        let _ = process.kill().await?;
    }

    Ok(())
}
