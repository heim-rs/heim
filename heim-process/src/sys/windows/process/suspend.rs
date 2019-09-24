use ntapi::ntkeapi;
use winapi::shared::ntdef;

use super::bindings;
use crate::{Pid, ProcessError, ProcessResult};

pub fn is_suspended(pid: Pid) -> ProcessResult<bool> {
    let pid_handle = pid as ntdef::HANDLE;
    let processes = bindings::processes::NtProcesses::load()?;
    let process = processes
        .iter()
        .find(|process| process.process.UniqueProcessId == pid_handle)
        .ok_or_else(|| ProcessError::NoSuchProcess(pid))?;

    let is_running = process.threads.iter().any(|thread| {
        thread.ThreadState != ntkeapi::Waiting || thread.WaitReason != ntkeapi::Suspended
    });

    Ok(!is_running)
}
