#![feature(await_macro, async_await, futures_api, test)]

extern crate test;

use heim_common::prelude::*;
use heim_common::units::si::time::second;
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
    let platform = await!(host::platform());
    assert!(platform.is_ok());

    let platform = platform.unwrap();
    let _ = platform.system();
    let _ = platform.release();
    let _ = platform.version();
    let _ = platform.architecture();
}

#[runtime::test]
async fn smoke_uptime() {
    let uptime = await!(host::uptime());

    assert!(uptime.is_ok());
    assert!(uptime.unwrap().get::<second>() > 0.0);
}

#[runtime::test]
async fn smoke_users() {
    let mut users = host::users();
    while let Some(user) = await!(users.next()) {
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
