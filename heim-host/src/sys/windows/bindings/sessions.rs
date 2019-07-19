use std::mem;

use winapi::um::winnt::PVOID;
use winapi::shared::minwindef::DWORD;

use heim_common::prelude::*;

use super::{wtsapi32, Session};

/// Iterator over user sessions
pub struct Sessions {
    info: wtsapi32::WTS_SESSION_INFOW,
    current: DWORD,
    count: DWORD,
}

impl Sessions {
    #[allow(trivial_casts)]
    pub fn new() -> Result<Sessions> {
        let mut info = mem::MaybeUninit::<wtsapi32::WTS_SESSION_INFOW>::uninit();
        let mut count: DWORD = 0;

        let result = unsafe {
            // https://docs.microsoft.com/en-us/windows/desktop/api/wtsapi32/nf-wtsapi32-wtsenumerateprocessesw
            wtsapi32::WTSEnumerateSessionsW(
                wtsapi32::WTS_CURRENT_SERVER_HANDLE,
                0,
                1,
                &mut &mut info as *mut &mut _ as *mut wtsapi32::PWTS_SESSION_INFOW,
                &mut count,
            )
        };

        dbg!("WTSEnumerateSessionsW", result);

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            let info = unsafe {
                info.assume_init()
            };

            Ok(Sessions {
                info,
                count,
                current: 0,
            })
        }
    }
}

// `wtsapi32::WTS_SESSION_INFOW` contains pointers in it,
// but since we are responsible to free this struct later,
// I think it is okay to `Send` it?
unsafe impl Send for Sessions {}

impl Drop for Sessions {
    #[allow(trivial_casts)]
    fn drop(&mut self) {
        unsafe {
            wtsapi32::WTSFreeMemory(&mut self.info as *mut _ as PVOID)
        }
    }
}

impl Iterator for Sessions {
    type Item = Session;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.count {
            None
        } else {
            let session: wtsapi32::WTS_SESSION_INFOW = unsafe {
                *(&mut self.info as wtsapi32::PWTS_SESSION_INFOW).offset(self.current as isize)
            };
            self.current += 1;

            Some(Session::new(session.SessionId))
        }
    }
}

//impl Iterator for Sessions {
//    type Item = Result<User>;
//
//    #[allow(trivial_casts)]
//    fn next(&mut self) -> Option<Self::Item> {
//        loop {
//            if self.current > self.count {
//                return None
//            }
//
//            let session: wtsapi32::WTS_SESSION_INFOW = unsafe {
//                *(&mut self.info as wtsapi32::PWTS_SESSION_INFOW).offset(self.current as isize)
//            };
//
//            // No matter of result returned later, this one entry was fetched
//            // and will not be accessed later
//            self.current += 1;
//
//            let mut session_info = Self::get_session_info(session.SessionId)?;
//            let address = Self::get_address(session.SessionId)?;
//
//            // Fast-skipping users with empty username
//            match session_info.UserName.iter().next() {
//                Some(0x00) | None => continue,
//                _ => {}
//            }
//
//            let username = Self::from_wide(&session_info.UserName);
//            let domain = Self::from_wide(&session_info.Domain);
//
//            unsafe {
//                wtsapi32::WTSFreeMemory(&mut session_info as *mut _ as PVOID);
//            }
//
//            return Some(Ok(User {
//                domain,
//                username,
//                address,
//            }))
//        }
//    }
//}
