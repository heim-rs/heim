use heim_common::Pid;
use std::path::{Path, PathBuf};

use heim_runtime as rt;

pub fn process_file_path<T: AsRef<Path>>(pid: Pid, filename: T) -> PathBuf {
    // It should be max up to 2^22, so 7 bytes should be enough to squeeze string representation of pid?
    static MAX_PID_LENGTH: usize = 7usize;
    let procfs_root = rt::linux::procfs_root();
    let required_len =
        procfs_root.as_os_str().len() + MAX_PID_LENGTH + filename.as_ref().as_os_str().len();
    let mut result = PathBuf::with_capacity(required_len);
    result.push(procfs_root);
    result.push(pid.to_string());
    result.push(filename.as_ref());

    result
}
