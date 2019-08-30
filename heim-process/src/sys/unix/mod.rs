use std::io;

pub fn pid_exists(pid: crate::Pid) -> bool {
    if pid == 0 {
        return true;
    }

    let result = unsafe { libc::kill(pid, 0) };

    if result == 0 {
        true
    } else {
        let e = io::Error::last_os_error();
        match e.raw_os_error() {
            Some(libc::ESRCH) => false,
            Some(libc::EPERM) => true,
            _ => true,
        }
    }
}
