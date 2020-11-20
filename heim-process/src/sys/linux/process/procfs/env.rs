use std::io;

use heim_runtime as rt;

use crate::{Pid, ProcessError, ProcessResult};

use crate::sys::linux::process::procfs::process_file_path;
pub use crate::sys::unix::{Environment, IntoEnvironmentIter};

pub async fn environment(pid: Pid) -> ProcessResult<Environment> {
    let path = process_file_path(pid, "environ");

    match rt::fs::read(path).await {
        Ok(contents) => Ok(Environment::from_bytes(&contents)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Err(ProcessError::NoSuchProcess(pid)),
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            Err(ProcessError::AccessDenied(pid))
        }
        Err(e) => Err(e.into()),
    }
}
