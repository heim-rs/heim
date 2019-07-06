#![feature(async_await, test)]

extern crate test;

use heim_common::prelude::*;
use heim_host as host;

cfg_if::cfg_if! {
    if #[cfg(all(unix, not(target_os = "openbsd")))] {
        use heim_host::os::unix::UserExt;
    } else if #[cfg(target_os = "windows")] {
        use heim_host::os::windows::UserExt;
    }
}

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

#[heim_derive::skip_ci(target_os = "windows")] // https://github.com/heim-rs/heim/issues/32
#[runtime::test]
async fn smoke_users() {
    let mut users = host::users();
    while let Some(user) = users.next().await {
        let user = user.unwrap();

        let _ = user.username();

        #[cfg(all(unix, not(target_os = "openbsd")))]
        let _ = user.terminal();

        #[cfg(target_os = "windows")]
        let _ = user.domain();
        #[cfg(target_os = "windows")]
        let _ = user.address();
    }
}
