use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
/// While utmp handling routines are all the same,
/// `utmpx` struct varies from platform to platform.
///
/// Problems:
///  * `getutxent` / `setutxent` / `endutxent` functions are not thread-safe
///  * `utmp` file format seriously varies from OS to OS (and from OS version to OS version too)
///
/// So, instead of a nice thread-safe interface for `Users` stream,
/// it will collect all entries during initialization, while we are running in a one thread.
///
/// This will hit the performance a little bit, but at least it would be a portable solution.
///
/// Also, musl functions are stubs:
/// https://wiki.musl-libc.org/faq.html#Q:_Why_is_the_utmp/wtmp_functionality_only_implemented_as_stubs?
///
/// See also:
///  * https://github.com/libyal/dtformats/blob/master/documentation/Utmp%20login%20records%20format.asciidoc
use std::ptr;

pub fn get_users<T: From<libc::utmpx>>() -> Vec<T> {
    // TODO: Should we try to guess the capacity?
    let mut users = Vec::with_capacity(1);
    unsafe {
        libc::setutxent();
        loop {
            let entry = libc::getutxent();
            if entry.is_null() {
                break;
            }

            if (*entry).ut_type != libc::USER_PROCESS {
                continue;
            }

            users.push(T::from(*entry))
        }
        libc::endutxent();
    }

    users
}

#[allow(unused, trivial_casts)]
pub(crate) fn from_ut_addr_v6(addr: &[i32; 4]) -> Option<IpAddr> {
    match addr {
        [0, 0, 0, 0] => None,
        [octet, 0, 0, 0] => Some(Ipv4Addr::from(*octet as u32).into()),
        octets => {
            let mut raw_ip: u128 = 0;
            unsafe {
                ptr::copy_nonoverlapping(octets.as_ptr(), &mut raw_ip as *mut u128 as *mut i32, 16);
            }
            Some(Ipv6Addr::from(raw_ip).into())
        }
    }
}
