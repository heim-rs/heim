use std::ptr;
use std::slice;

use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::PVOID;

use heim_common::prelude::*;

use super::super::bindings::wtsapi32;
use super::Session;

/// Iterator over user sessions.
pub struct Sessions<'s> {
    info: &'s mut [wtsapi32::WTS_SESSION_INFOW],
    current: DWORD,
    count: DWORD,
}

impl<'s> Sessions<'s> {
    #[allow(trivial_casts)]
    pub fn new() -> Result<Sessions<'s>> {
        let mut info = ptr::null_mut();
        let mut count: DWORD = 0;

        let result = unsafe {
            // https://docs.microsoft.com/en-us/windows/desktop/api/wtsapi32/nf-wtsapi32-wtsenumerateprocessesw
            wtsapi32::WTSEnumerateSessionsW(
                wtsapi32::WTS_CURRENT_SERVER_HANDLE,
                0,
                1,
                &mut info,
                &mut count,
            )
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            let sessions = unsafe { slice::from_raw_parts_mut(info, count as usize) };

            Ok(Sessions {
                info: sessions,
                count,
                current: 0,
            })
        }
    }
}

// `wtsapi32::WTS_SESSION_INFOW` contains pointers in it,
// but since we are responsible to free this struct later,
// I think it is okay to `Send` it?
unsafe impl<'s> Send for Sessions<'s> {}

impl<'s> Drop for Sessions<'s> {
    #[allow(trivial_casts)]
    fn drop(&mut self) {
        unsafe {
            // https://docs.microsoft.com/en-us/windows/win32/api/wtsapi32/nf-wtsapi32-wtsfreememory
            wtsapi32::WTSFreeMemory(self.info.as_mut_ptr() as *mut _ as PVOID)
        }
    }
}

impl<'s> Iterator for Sessions<'s> {
    type Item = Session;

    #[allow(trivial_casts)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.count {
            let session = self
                .info
                .get(self.current as usize)
                .expect("Invalid session index");
            self.current += 1;

            Some(Session::new(session.SessionId))
        } else {
            None
        }
    }
}
