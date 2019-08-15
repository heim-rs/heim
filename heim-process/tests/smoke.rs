#![feature(async_await)]

use heim_common::prelude::*;
use heim_process as process;

#[heim_derive::test]
async fn smoke_pid_exists() {
    let result = process::pid_exists(1).await;

    assert!(result.is_ok());
}

#[heim_derive::test]
async fn smoke_pids() {
    let mut pids = process::pids();

    while let Some(pid) = pids.next().await {
        assert!(pid.is_ok());
    }
}

#[heim_derive::test]
async fn smoke_processes() {
    let mut processes = process::processes();

    while let Some(process) = processes.next().await {
        let process = process.unwrap();

        let _ = process.pid();
        let _ = process.parent_pid().await;
        let _ = process.name().await;
        let _ = process.exe().await;
        let _ = process.status().await;
        let _ = process.cpu_time().await;
        let _ = process.memory().await;

        let _ = process
            .net_io_counters()
            .try_for_each(|_| future::ok(()))
            .await;

        #[cfg(target_os = "linux")]
        {
            use heim_process::os::linux::ProcessExt;

            let _ = process.io_counters().await;
        }
    }
}
