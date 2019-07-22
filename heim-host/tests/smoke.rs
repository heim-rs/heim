#![feature(async_await, test)]

extern crate test;

use heim_common::prelude::*;
use heim_host as host;

#[runtime::test]
async fn smoke_platform() {
    let platform = host::platform().await;
    let platform = platform.unwrap();
    let _ = platform.system();
    let _ = platform.release();
    let _ = platform.version();
    let _ = platform.architecture();
}

#[runtime::test]
async fn smoke_uptime() {
    let uptime = host::uptime().await;

    assert!(uptime.unwrap().get() > 0.0);
}

#[runtime::test]
async fn smoke_users() {
    let mut users = host::users();
    while let Some(user) = users.next().await {
        let user = user.unwrap();

        let _ = user.username();
    }
}
