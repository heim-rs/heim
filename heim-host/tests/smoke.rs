use heim_common::units::si::time::second;
use heim_host as host;
use heim_runtime::{self as runtime, SyncRuntime};

cfg_if::cfg_if! {
    if #[cfg(all(unix, not(target_os = "openbsd")))] {
        use heim_host::os::unix::UserExt;
    } else if #[cfg(target_os = "windows")] {
        use heim_host::os::windows::UserExt;
    }
}

#[test]
fn smoke_platform() {
    let mut rt = runtime::new().unwrap();
    let platform = rt.block_run(host::platform());
    assert!(platform.is_ok());

    let platform = platform.unwrap();
    let _ = platform.system();
    let _ = platform.release();
    let _ = platform.version();
    let _ = platform.architecture();
}

#[test]
fn smoke_uptime() {
    let mut rt = runtime::new().unwrap();
    let uptime = rt.block_run(host::uptime());

    assert!(uptime.is_ok());
    assert!(uptime.unwrap().get::<second>() > 0.0);
}

#[test]
fn smoke_users() {
    let mut rt = runtime::new().unwrap();
    let users = rt.block_collect(host::users());

    for user in users.flatten() {
        let _ = user.username();

        #[cfg(all(unix, not(target_os = "openbsd")))]
        let _ = user.terminal();

        #[cfg(target_os = "windows")]
        let _ = user.domain();
        #[cfg(target_os = "windows")]
        let _ = user.address();
    }
}
