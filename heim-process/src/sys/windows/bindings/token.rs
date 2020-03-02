use std::ptr;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::{TokenUser, HANDLE, TOKEN_QUERY, TOKEN_USER};

use heim_common::prelude::*;
use heim_common::sys::windows::Handle;
use heim_common::Result;
use heim_host::os::windows::UserExt;
use heim_host::User;

pub struct Token(Handle);

impl Token {
    pub fn open(process_handle: &Handle) -> Result<Self> {
        let mut token_handle: HANDLE = ptr::null_mut();

        let result = unsafe { OpenProcessToken(**process_handle, TOKEN_QUERY, &mut token_handle) };

        if result == 0 {
            return Err(Error::last_os_error().with_ffi("OpenProcessToken"));
        }

        Ok(Self(Handle::new(token_handle)))
    }

    pub fn user(&self) -> Result<User> {
        // data should always be 44 bytes
        let mut data: Vec<u8> = Vec::with_capacity(64);
        let mut length: DWORD = 0;

        let result = unsafe {
            GetTokenInformation(
                *self.0,
                TokenUser,
                data.as_mut_ptr() as LPVOID,
                data.capacity() as DWORD,
                &mut length,
            )
        };

        if result == 0 {
            return Err(Error::last_os_error().with_ffi("GetTokenInformation"));
        }

        unsafe { data.set_len(length as usize) };

        let token_user = unsafe { ptr::read(data.as_ptr() as *const TOKEN_USER) };

        User::try_from_sid(&token_user.User)
    }
}
