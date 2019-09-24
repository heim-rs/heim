#![cfg(unix)]

use std::process::{Command, Stdio};

use heim_process as process;

#[heim_derive::test]
async fn test_kill() {
    let yes_path = match which::which("yes") {
        Ok(path) => path,
        Err(e) => {
            eprintln!(
                "Unable to find `yes` command, signals test will be skipped: {:?}",
                e
            );
            return;
        }
    };

    let mut child = Command::new(yes_path)
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    let process = process::get(child.id() as process::Pid).await.unwrap();
    let result = process.kill().await;
    assert!(result.is_ok(), "Failed to kill the process: {:?}", result);

    // Technically, child process will be in a zombie status at this exact moment,
    // as it was killed, but the parent process (we are) still monitoring it.
    // It will be terminated properly in the `child.try_wait`.

    match child.try_wait() {
        Ok(..) => {
            // Child exited, assuming that we did it
        }
        Err(e) => panic!("Process::signal failed to kill the test process: {:#?}", e),
    }
}
