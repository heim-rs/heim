use std::path::PathBuf;

use heim_common::prelude::StreamExt;
use heim_process as process;

#[heim_derive::main]
async fn main() -> Result<(), process::ProcessError> {
    let mut processes = process::processes();
    println!(
        "| {:6} | {:6} | {:10} | {:40} | {:50} |",
        "pid", "ppid", "status", "name", "exe"
    );

    while let Some(process) = processes.next().await {
        let process = process?;

        println!(
            "| {:6} | {:6} | {:10?} | {:40} | {:50?}",
            process.pid(),
            process.parent_pid().await.unwrap_or(0),
            process.status().await.unwrap(),
            process.name().await.unwrap_or_else(|_| "".to_string()),
            process.exe().await.unwrap_or_else(|_| PathBuf::new()),
        );
    }

    Ok(())
}
