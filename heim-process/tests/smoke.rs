#![allow(stable_features)]
#![feature(async_await, futures_api)]

use heim_common::prelude::*;
use heim_process as process;

#[runtime::test]
async fn smoke_pids() {
    let mut pids = process::pids();

    while let Some(pid) = pids.next().await {
        assert!(pid.is_ok());
    }
}
