use std::{convert::TryFrom, ffi::CStr};

use heim_common::prelude::*;
use heim_common::{Pid, Uid};

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

impl From<*mut libc::passwd> for User {
    fn from(entry: *mut libc::passwd) -> Self {
        let username = unsafe {
            CStr::from_ptr((*entry).pw_name)
                .to_string_lossy()
                .into_owned()
        };

        User {
            username,
            id: "".to_string(),
            terminal: "".to_string(),
            hostname: "".to_string(),
            pid: 0,
        }
    }
}

impl TryFrom<Uid> for User {
    type Error = Error;
    fn try_from(uid: Uid) -> Result<Self> {
        let passwd = unsafe { libc::getpwuid(uid) };
        if passwd.is_null() {
            return Err(Error::last_os_error().with_ffi("getpwuid"));
        }
        let user = User::from(passwd);
        Ok(user)
    }
}

pub async fn users() -> Result<impl Stream<Item = Result<User>>> {
    let users = get_users::<User>();

    Ok(stream::iter(users).map(Ok))
}
