use std::mem;
use std::ptr;
use std::pin::Pin;
use std::task::{Poll, Context};
use std::net::{IpAddr, Ipv4Addr};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::um::winnt::{PVOID, WCHAR, LPWSTR};
use winapi::shared::minwindef::DWORD;
use winapi::shared::ws2def::{AF_INET, AF_INET6, AF_IPX, AF_NETBIOS, AF_UNSPEC};

use heim_common::prelude::*;

use super::wtsapi32;

pub struct User {
    domain: String,
    username: String,
    address: Option<IpAddr>,
}

impl User {
    pub fn domain(&self) -> &str {
        self.domain.as_str()
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn address(&self) -> Option<&IpAddr> {
        self.address.as_ref()
    }
}

pub struct Sessions {
    info: wtsapi32::WTS_SESSION_INFOW,
    current: DWORD,
    count: DWORD,
}

// `wtsapi32::WTS_SESSION_INFOW` contains pointers in it,
// but since we are responsible to free this struct later,
// I think it is okay to `Send` it?
unsafe impl Send for Sessions {}

impl Sessions {
    pub fn new() -> Result<Sessions> {
        // TODO: Use MaybeUninit here
        let mut info: wtsapi32::WTS_SESSION_INFOW = unsafe { mem::zeroed() };
        let mut count = 0;

        let result = unsafe {
            // https://docs.microsoft.com/en-us/windows/desktop/api/wtsapi32/nf-wtsapi32-wtsenumerateprocessesw
            wtsapi32::WTSEnumerateSessionsW(
                wtsapi32::WTS_CURRENT_SERVER_HANDLE,
                0,
                1,
                &mut &mut info as *mut &mut _ as *mut wtsapi32::PWTS_SESSION_INFOW,
                &mut count as *mut DWORD,
            )
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(Sessions {
                info,
                count,
                current: 0,
            })
        }
    }

    // https://docs.microsoft.com/ru-ru/windows/desktop/api/wtsapi32/ns-wtsapi32-_wtsinfow
    fn get_session_info(session_id: DWORD) -> Result<wtsapi32::WTSINFOW> {
        let mut buffer: wtsapi32::PWTSINFOW = ptr::null_mut();
        let mut bytes: DWORD = 0;
        let result = unsafe {
            wtsapi32::WTSQuerySessionInformationW(
                wtsapi32::WTS_CURRENT_SERVER_HANDLE,
                session_id,
                wtsapi32::WTSSessionInfo,
                &mut buffer as *mut wtsapi32::PWTSINFOW as *mut LPWSTR,
                &mut bytes,
            )
        };

        if result == 0 {
            return Err(Error::last_os_error())
        }

        unsafe {
            Ok(*buffer)
        }
    }

    fn get_address(session_id: DWORD) -> Result<Option<IpAddr>> {
        let mut address_ptr: wtsapi32::PWTS_CLIENT_ADDRESS = ptr::null_mut();
        let mut address_bytes: DWORD = 0;
        let result = unsafe {
            wtsapi32::WTSQuerySessionInformationW(
                wtsapi32::WTS_CURRENT_SERVER_HANDLE,
                session_id,
                wtsapi32::WTSClientAddress,
                &mut address_ptr as *mut _ as *mut LPWSTR,
                &mut address_bytes,
            )
        };

        if result == 0 {
            return Err(Error::last_os_error())
        }

        let address = match unsafe { (*address_ptr).AddressFamily as i32 } {
            AF_INET => {
                let bytes = unsafe { (*address_ptr).Address };
                Some(IpAddr::V4(Ipv4Addr::new(bytes[2], bytes[3], bytes[4], bytes[5])))
            },
            AF_INET6 => {
                let bytes = unsafe { (*address_ptr).Address };
                let mut addr: [u8; 16] = [0; 16];
                addr.copy_from_slice(&bytes[2..18]);

                Some(IpAddr::from(addr))
            },

            // TODO: Implement address parsing for the following families
            // See `crate::os::windows::UserExt::address` comments additionally
            AF_IPX=> None,
            AF_NETBIOS=> None,
            AF_UNSPEC => None,

            other => unreachable!("Unknown family {}", other),
        };

        Ok(address)
    }

    fn from_wide(chars: &[WCHAR]) -> String {
        let first_null = chars.iter().position(|c| *c == 0x00).unwrap_or(0);
        OsString::from_wide(&chars[..first_null]).to_string_lossy().to_string()
    }
}

impl Drop for Sessions {
    fn drop(&mut self) {
        unsafe {
            wtsapi32::WTSFreeMemory(&mut self.info as *mut _ as PVOID)
        }
    }
}

impl Stream for Sessions {
    type Item = Result<User>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context) -> Poll<Option<Self::Item>> {
        loop {
            if self.current >= self.count {
                return Poll::Ready(None);
            }

            let session: wtsapi32::WTS_SESSION_INFOW = unsafe {
                *(&mut self.info as wtsapi32::PWTS_SESSION_INFOW).offset(self.current as isize)
            };

            // No matter of result returned later, this one entry was fetched
            // and will not be accessed later
            self.current += 1;

            let mut session_info = Self::get_session_info(session.SessionId)?;
            let address = Self::get_address(session.SessionId)?;

            // Fast-skipping users with empty username
            match session_info.UserName.iter().next() {
                Some(0x00) | None => continue,
                _ => {}
            }

            let username = Self::from_wide(&session_info.UserName);
            let domain = Self::from_wide(&session_info.Domain);

            unsafe {
                wtsapi32::WTSFreeMemory(&mut session_info as *mut _ as PVOID);
            }

            return Poll::Ready(Some(Ok(User {
                domain,
                username,
                address,
            })))
        }
    }
}

pub fn users() -> impl Stream<Item=Result<User>> {
    future::lazy(|_| {
        Sessions::new()
    })
    .map_ok(|sessions| {
        Box::pin(sessions) as Pin<Box<dyn Stream<Item = _> + Send>>
    })
    .unwrap_or_else(|e| {
        Box::pin(stream::once(future::err(e)))
    })
    .flatten_stream()
}
