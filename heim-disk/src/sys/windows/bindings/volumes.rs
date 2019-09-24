use std::ffi::OsString;
use std::io;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use winapi::shared::{minwindef, winerror};
use winapi::um::{fileapi, handleapi, winnt};

use heim_common::prelude::*;

// `winerror::ERROR_NO_MORE_FILES` is `u32`, but `io::Error` operates with `i32`
const ERROR_NO_MORE_FILES: i32 = winerror::ERROR_NO_MORE_FILES as i32;

/// Iterator over Windows volumes.
pub struct Volumes {
    handle: Option<winnt::HANDLE>, // From the FindFirstVolumeW
    buffer: [winnt::WCHAR; minwindef::MAX_PATH],
}

impl Volumes {
    pub fn new() -> Self {
        Volumes {
            handle: None,
            buffer: [0x00; minwindef::MAX_PATH],
        }
    }

    fn find_first_volume(&mut self) -> Result<PathBuf> {
        let handle = unsafe {
            fileapi::FindFirstVolumeW(
                self.buffer.as_mut_ptr(),
                minwindef::MAX_PATH as minwindef::DWORD,
            )
        };

        if handle == handleapi::INVALID_HANDLE_VALUE {
            Err(Error::last_os_error())
        } else {
            let first_null = self
                .buffer
                .iter()
                .position(|byte| *byte == 0x00)
                .unwrap_or(0);

            let path_str = OsString::from_wide(&self.buffer[..first_null]);

            self.handle = Some(handle);

            Ok(PathBuf::from(path_str))
        }
    }

    fn find_next_volume(&mut self, handle: winnt::HANDLE) -> Result<Option<PathBuf>> {
        let result = unsafe {
            fileapi::FindNextVolumeW(
                handle,
                self.buffer.as_mut_ptr(),
                minwindef::MAX_PATH as minwindef::DWORD,
            )
        };

        if result != 0 {
            // Next volume was found
            let first_null = self
                .buffer
                .iter()
                .position(|byte| *byte == 0x00)
                .unwrap_or(0);

            let path_str = OsString::from_wide(&self.buffer[..first_null]);

            Ok(Some(PathBuf::from(path_str)))
        } else {
            // Either we caught some error or there are no more volumes to iterate
            let error = io::Error::last_os_error();
            match error.raw_os_error() {
                // Iteration ended
                Some(ERROR_NO_MORE_FILES) => Ok(None),
                // Some error
                _ => Err(error.into()),
            }
        }
    }
}

// MSDN says nothing about thread-safety of the `HANDLE`
// returned by the `FindFirstVolumeW`, so I'm going to assume
// that it is okay to "Send" it, since we are using it only
// in a `Stream` and it should not be used in parallel by multiple threads
unsafe impl Send for Volumes {}

impl Drop for Volumes {
    fn drop(&mut self) {
        if let Some(handle) = self.handle {
            let result = unsafe { fileapi::FindVolumeClose(handle) };

            assert!(result != 0, "Unable to close volumes handle");
        }
    }
}

impl Iterator for Volumes {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.handle {
            // Search was not started yet
            None => match self.find_first_volume() {
                Err(e) => Some(Err(e)),
                Ok(path) => Some(Ok(path)),
            },
            // Continuing search
            Some(handle) => match self.find_next_volume(handle) {
                Ok(None) => None,
                Ok(Some(path)) => Some(Ok(path)),
                Err(e) => Some(Err(e)),
            },
        }
    }
}
