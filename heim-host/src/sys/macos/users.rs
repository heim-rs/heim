use std::ffi::CStr;

use heim_common::prelude::*;
use heim_common::Pid;

use super::super::unix::get_users;

#[derive(Debug)]
pub struct User {
    username: String,
    terminal: String,
    id: String,
    hostname: String,
    pid: Pid,
}

impl User {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn terminal(&self) -> &str {
        &self.terminal
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

impl From<libc::utmpx> for User {
    fn from(entry: libc::utmpx) -> User {
        let username = unsafe {
            CStr::from_ptr(entry.ut_user.as_ptr())
                .to_string_lossy()
                .into_owned()
        };
        let terminal = unsafe {
            CStr::from_ptr(entry.ut_line.as_ptr())
                .to_string_lossy()
                .into_owned()
        };
        let id = unsafe {
            CStr::from_ptr(entry.ut_id.as_ptr())
                .to_string_lossy()
                .into_owned()
        };
        let hostname = unsafe {
            CStr::from_ptr(entry.ut_host.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        User {
            username,
            terminal,
            id,
            hostname,
            pid: entry.ut_pid,
        }
    }
}

pub fn users() -> impl Stream<Item = Result<User>> {
    future::lazy(|_| {
        let users = get_users::<User>();

        Ok(stream::iter(users).map(Ok))
    })
    .try_flatten_stream()
}
