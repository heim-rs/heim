use heim_common::prelude::*;
use heim_process as process;
use heim_process::ProcessError;
use heim_runtime as rt;

#[heim_derive::test]
async fn smoke_pid_exists() {
    let result = process::pid_exists(1).await;

    assert!(result.is_ok());
}

#[heim_derive::test]
async fn smoke_pids() {
    let pids = process::pids();
    rt::pin!(pids);

    while let Some(pid) = pids.next().await {
        assert!(pid.is_ok());
    }
}

/// Try to .await the `Process` method and panic if there is a loading error occured.
///
/// Both successful results and
/// NoSuchProcess/AccessDenied/ZombieProcess errors
/// are going to be ignored, as we can't guarantee
/// that queried process will be okay during the testing.
macro_rules! try_method {
    ($method:expr) => {
        if let Err(ProcessError::Load(e)) = $method.await {
            assert!(
                false,
                "`{}` method returned an error: {:#?}",
                stringify!($method),
                e
            );
        }
    };
}

#[heim_derive::test]
async fn smoke_processes() {
    let processes = process::processes();
    rt::pin!(processes);

    while let Some(process) = processes.next().await {
        let process = match process {
            Ok(process) => process,
            e @ Err(ProcessError::Load(..)) => panic!("{:#?}", e),
            _ => continue,
        };

        let _ = process.pid();
        try_method!(process.parent_pid());
        try_method!(process.name());
        try_method!(process.command());
        try_method!(process.exe());
        #[cfg(not(target_os = "windows"))] // Not implemented yet
        try_method!(process.cwd());
        try_method!(process.status());
        #[cfg(any(target_os = "linux", target_os = "macos"))] // Not implemented yet for all platforms
        try_method!(process.environment());
        try_method!(process.create_time());
        try_method!(process.cpu_time());
        try_method!(process.cpu_usage());
        try_method!(process.memory());
        try_method!(process.is_running());

        #[cfg(unix)]
        {
            use heim_process::os::unix::ProcessExt;

            try_method!(process.niceness());
        }

        #[cfg(target_os = "linux")]
        {
            use heim_process::os::linux::ProcessExt;

            try_method!(process.io_counters());
            try_method!(process.net_io_counters().try_for_each(|_| future::ok(())));
        }

        #[cfg(target_os = "windows")]
        {
            use heim_process::os::windows::ProcessExt;

            try_method!(process.priority());
        }
    }
}
