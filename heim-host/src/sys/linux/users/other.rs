use std::ffi::CStr;
use std::net::IpAddr;

use heim_common::prelude::*;
use heim_common::Pid;

use crate::sys::unix::{from_ut_addr_v6, get_users};

#[derive(Debug)]
pub struct User {
    username: String,
    terminal: String,
    id: String,
    hostname: String,
    pid: libc::pid_t,
    addr: Option<IpAddr>,
    session_id: i32,
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

    pub fn address(&self) -> Option<IpAddr> {
        self.addr
    }

    pub fn session_id(&self) -> i32 {
        self.session_id
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
            session_id: entry.ut_session,
            addr: from_ut_addr_v6(&entry.ut_addr_v6),
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
