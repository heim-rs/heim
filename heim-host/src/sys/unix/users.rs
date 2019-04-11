// Problems:
//  * `getutxent` / `setutxent` / `endutxent` functions are not thread-safe
//  * `utmp` file format seriously varies from OS to OS (and from OS version to OS version too)
//
// So, instead of a nice thread-safe interface for `Users` stream,
// it will collect all entries during initialization, while we are running in a one thread.
//
// This will hit the performance a little bit, but at least it would be a portable solution.
//
// Also, musl functions are stubs:
// https://wiki.musl-libc.org/faq.html#Q:_Why_is_the_utmp/wtmp_functionality_only_implemented_as_stubs?

use heim_common::prelude::*;

use super::into_cow;

pub struct User {
    username: String,
    terminal: String,
    pid: libc::pid_t,
}

impl User {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn terminal(&self) -> Option<&str> {
        Some(&self.terminal)
    }

    pub fn pid(&self) -> libc::pid_t {
        self.pid
    }
}

impl From<libc::utmpx> for User {
    fn from(entry: libc::utmpx) -> User {
        User{
            username: unsafe { into_cow(&entry.ut_user).into_owned() },
            terminal: unsafe { into_cow(&entry.ut_line).into_owned() },
            pid: entry.ut_pid,
        }
    }
}

//ut_type: c_short
//ut_pid: pid_t
//ut_line: [c_char; 32]
//ut_id: [c_char; 4]
//ut_user: [c_char; 32]
//ut_host: [c_char; 256]
//ut_exit: __exit_status
//ut_session: int32_t
//ut_tv: __timeval
//ut_addr_v6: [int32_t; 4]

pub fn users() -> impl Stream<Item=User, Error=Error> {
    // TODO: Should we try to guess the capacity?
    let mut users = vec![];
    unsafe {
        libc::setutxent();
        loop {
            let entry = libc::getutxent();
            if entry.is_null() {
                break
            }

            users.push(User::from(*entry))
        }
        libc::endutxent();
    }

    stream::iter_ok(users)
}
