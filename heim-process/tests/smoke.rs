#![feature(async_await)]

use heim_common::prelude::*;
use heim_process as process;

#[runtime::test]
async fn smoke_pid_exists() {
    let result = process::pid_exists(1).await;

    assert!(result.is_ok());
}

#[runtime::test]
async fn smoke_pids() {
    let mut pids = process::pids();

    while let Some(pid) = pids.next().await {
        assert!(pid.is_ok());
    }
}
