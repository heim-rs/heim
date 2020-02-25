use std::ops;

use winapi::shared::ntdef::HANDLE;
use winapi::um::handleapi;

use crate::{Error, Result};

/// Safe wrapper for windows `HANDLE` type.
#[derive(Debug)]
pub struct Handle(HANDLE);

impl Handle {
    pub fn new(h: HANDLE) -> Self {
        assert_ne!(h, handleapi::INVALID_HANDLE_VALUE);

        Self(h)
    }

    pub fn close(self) -> Result<()> {
        let result = unsafe { handleapi::CloseHandle(self.0) };

        if result == 0 {
            Err(Error::last_os_error().with_ffi("Closehandle"))
        } else {
            Ok(())
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let result = unsafe { handleapi::CloseHandle(self.0) };

        assert!(
            result != 0,
            "{:?}",
            Error::last_os_error().with_ffi("CloseHandle")
        );
    }
}

impl ops::Deref for Handle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HANDLE> for Handle {
    fn from(h: HANDLE) -> Handle {
        Self::new(h)
    }
}
