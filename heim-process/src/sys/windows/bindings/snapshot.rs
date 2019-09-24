use std::io;
use std::mem;

use winapi::shared::{minwindef, winerror};
use winapi::um::{handleapi, tlhelp32, winnt};

use heim_common::prelude::*;

/// Processes snapshot.
pub struct Snapshot {
    handle: winnt::HANDLE,
    first: bool,
}

impl Snapshot {
    pub fn new() -> Result<Snapshot> {
        let handle = unsafe { tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, 0) };
        if handle == handleapi::INVALID_HANDLE_VALUE {
            Err(Error::last_os_error())
        } else {
            Ok(Snapshot {
                handle,
                first: true,
            })
        }
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        let result = unsafe { handleapi::CloseHandle(self.handle) };
        debug_assert!(result != 0);
    }
}

impl Iterator for Snapshot {
    type Item = Result<tlhelp32::PROCESSENTRY32W>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = mem::MaybeUninit::<tlhelp32::PROCESSENTRY32W>::uninit();
        let size = mem::size_of::<tlhelp32::PROCESSENTRY32W>() as minwindef::DWORD;
        unsafe {
            (*entry.as_mut_ptr()).dwSize = size;
        }

        let result = if self.first {
            self.first = false;

            unsafe { tlhelp32::Process32FirstW(self.handle, entry.as_mut_ptr()) }
        } else {
            unsafe { tlhelp32::Process32NextW(self.handle, entry.as_mut_ptr()) }
        };

        if result == 1 {
            let entry = unsafe { entry.assume_init() };
            Some(Ok(entry))
        } else {
            let e = io::Error::last_os_error();
            match e.raw_os_error() {
                Some(code) if code as u32 == winerror::ERROR_NO_MORE_FILES => None,
                _ => Some(Err(e.into())),
            }
        }
    }
}
