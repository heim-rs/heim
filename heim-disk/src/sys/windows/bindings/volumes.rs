use std::io;
use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::um::{winnt, fileapi, handleapi};
use winapi::shared::{minwindef, winerror};

use heim_common::prelude::*;

// `winerror::ERROR_NO_MORE_FILES` is `u32`, but `io::Error` operates with `i32`
const ERROR_NO_MORE_FILES: i32 = winerror::ERROR_NO_MORE_FILES as i32;

/// Iterator over Windows volumes.
pub struct Volumes {
    handle: Option<winnt::HANDLE>,  // From the FindFirstVolumeW
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
                minwindef::MAX_PATH as minwindef::DWORD
            )
        };

        if handle == handleapi::INVALID_HANDLE_VALUE {
            Err(Error::last_os_error())
        } else {
            let first_null = self.buffer.iter()
                .position(|byte| *byte == 0x00).unwrap_or(0);

            // What does that strange and mysterious `- 1` means?
            //
            // `PathBuf` yielded by this iterator will be passed into a
            // Windows' `CreateFile` later
            // and if "Volume GUID path" as in our case ends with a backslash `\`
            // (hint: it ends with it here), `CreateFile` assumes then that
            // we are opening not the volume itself, but a root directory on it.
            //
            // And we need to open volume in order to get performance statistics.
            //
            // Soo.. Since this iterator used only here, we can trim
            // the trailing backslash in a very efficient manner.
            //
            // See also: https://docs.microsoft.com/en-us/windows/desktop/fileio/naming-a-volume
            // The same thing is explained somewhere at the end of the page.
            let path_str = OsString::from_wide(&self.buffer[..first_null - 1]);

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
            let first_null = self.buffer.iter()
                .position(|byte| *byte == 0x00).unwrap_or(0);

            // For a `- 1` explanation see a big comment above
            let path_str = OsString::from_wide(&self.buffer[..first_null - 1]);

            Ok(Some(PathBuf::from(path_str)))

        } else {
            // Either we caught some error or there are no more volumes to iterate
            let error = io::Error::last_os_error();
            match error.raw_os_error() {
                // Iteration ended
                Some(ERROR_NO_MORE_FILES) | Some(0) => Ok(None),
                // Some error
                _ => Err(error.into())
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
            // TODO: Handle the "handle" closing failure (warn! at least?)
            let _ = unsafe {
                fileapi::FindVolumeClose(handle)
            };
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
