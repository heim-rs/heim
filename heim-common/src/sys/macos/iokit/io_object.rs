use std::convert::AsRef;
use std::mem;

use core_foundation::base::kCFAllocatorDefault;
use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::CFMutableDictionaryRef;
use core_foundation::dictionary::{CFDictionary, CFMutableDictionary};
use core_foundation::string::CFString;
use mach::kern_return;

use super::ffi;
use crate::{Error, Result};

/// Safe wrapper around the IOKit `io_object_t` type.
#[derive(Debug)]
pub struct IoObject(ffi::io_object_t);

impl IoObject {
    /// Returns a typed dictionary with this object's properties.
    pub fn properties(&self) -> Result<CFDictionary<CFString, CFType>> {
        unsafe {
            let mut props = mem::MaybeUninit::<CFMutableDictionaryRef>::uninit();

            let result = ffi::IORegistryEntryCreateCFProperties(
                self.0,
                props.as_mut_ptr(),
                kCFAllocatorDefault,
                0,
            );
            if result != kern_return::KERN_SUCCESS {
                Err(Error::last_os_error())
            } else {
                let props = props.assume_init();
                Ok(CFMutableDictionary::wrap_under_create_rule(props).to_immutable())
            }
        }
    }

    /// Gets the parent `io_object_t` for this `io_object_t` if there is one.
    pub fn parent(&self, plane: &[u8]) -> Result<IoObject> {
        let mut parent = mem::MaybeUninit::<ffi::io_object_t>::uninit();

        let result = unsafe {
            ffi::IORegistryEntryGetParentEntry(
                self.0,
                plane.as_ref().as_ptr() as *const libc::c_char,
                parent.as_mut_ptr(),
            )
        };

        if result != kern_return::KERN_SUCCESS {
            Err(Error::last_os_error())
        } else {
            let parent = unsafe { parent.assume_init() };
            Ok(parent.into())
        }
    }

    /// `class_name` should look like `b"IOBlockStorageDriver\0"` --
    /// a binary string with a trailing `0x00` char
    pub fn conforms_to(&self, class_name: &[u8]) -> bool {
        let result =
            unsafe { ffi::IOObjectConformsTo(self.0, class_name.as_ptr() as *const libc::c_char) };

        result != 0
    }
}

impl From<ffi::io_object_t> for IoObject {
    fn from(obj: ffi::io_object_t) -> IoObject {
        IoObject(obj)
    }
}

impl Drop for IoObject {
    fn drop(&mut self) {
        let result = unsafe { ffi::IOObjectRelease(self.0) };
        assert_eq!(result, kern_return::KERN_SUCCESS);
    }
}
