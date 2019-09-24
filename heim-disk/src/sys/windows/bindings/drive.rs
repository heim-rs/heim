use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::str::FromStr;

use winapi::ctypes::wchar_t;
use winapi::shared::minwindef::{DWORD, MAX_PATH};
use winapi::um::{errhandlingapi, fileapi, winbase};

use crate::os::windows::DriveType;
use crate::os::windows::Flags;
use crate::FileSystem;
use heim_common::prelude::{Error, Result};

// According to winapi docs 50 is a reasonable length to accomodate the volume path
// https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumenameforvolumemountpointw
const VOLUME_MAX_LEN: DWORD = 50;

/// Represents the logical drive.
///
/// Vec inside should contain path like `C:\\\0`,
/// four `wchar_t`s: drive letter, colon, backslash and a zero char.
#[derive(Debug)]
pub struct Drive([wchar_t; 4]);

impl Drive {
    /// Returns the inner buffer as a `PathBuf` (ex. `C:\`) withour trailing zero char.
    pub fn to_path_buf(&self) -> PathBuf {
        // Skipping trailing zero char also
        PathBuf::from(OsString::from_wide(&self.0[..(self.0.len() - 1)]))
    }

    pub fn volume_name(&self) -> Result<OsString> {
        let mut volume: Vec<wchar_t> = Vec::with_capacity(VOLUME_MAX_LEN as usize);
        let result = unsafe {
            fileapi::GetVolumeNameForVolumeMountPointW(
                self.0.as_ptr(),
                volume.as_mut_ptr(),
                VOLUME_MAX_LEN,
            )
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            unsafe {
                volume.set_len(VOLUME_MAX_LEN as usize);
            }

            let str_end = volume.iter().position(|chr| *chr == 0x00).unwrap_or(0);
            volume.truncate(str_end);
            let volume_path = OsString::from_wide(&volume);

            Ok(volume_path)
        }
    }

    /// ## Returns
    ///
    /// `Ok(Some(..))` - successful info fetch
    /// `Ok(None)` - disk should be ignored
    /// `Err(..)` - whoops
    pub fn information(&self) -> Result<Option<(Option<DriveType>, Flags, FileSystem)>> {
        let drive_type = DriveType::from_slice(&self.0);

        let mut flags: DWORD = 0;
        let mut fs_type: Vec<wchar_t> = Vec::with_capacity(MAX_PATH + 1);
        let mut old_mode: DWORD = 0;

        let err_mode_result = unsafe {
            errhandlingapi::SetThreadErrorMode(winbase::SEM_FAILCRITICALERRORS, &mut old_mode)
        };
        if err_mode_result == 0 {
            return Err(Error::last_os_error());
        }

        let result = unsafe {
            fileapi::GetVolumeInformationW(
                self.0.as_ptr(),
                ptr::null_mut(),
                // Originally Windows `ARRAYSIZE` macro is used here,
                // and we need to calculate length in bytes (while having u16 chars)
                (self.0.len() * 2) as DWORD,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut flags,
                fs_type.as_mut_ptr(),
                fs_type.capacity() as DWORD,
            )
        };

        let err_mode_result =
            unsafe { errhandlingapi::SetThreadErrorMode(old_mode, ptr::null_mut()) };
        if err_mode_result == 0 {
            return Err(Error::last_os_error());
        }

        match result {
            // Same to `psutil` and `gopsutil` we are going to ignore any errors
            // from the CDRoms, floppies and other removable disks
            0 if drive_type == Some(DriveType::CdRom) => Ok(None),
            0 if drive_type == Some(DriveType::Removable) => Ok(None),
            0 => Err(Error::last_os_error()),
            _ => {
                let flags = Flags::from_bits_truncate(flags);

                // So, since `GetVolumeInformationW` does not returns how much bytes
                // it wrote into a passed buffer, we need to find that manually.
                //
                // Quite unsafe, because we are going to poke some random memory here.
                // It is still in the range of pre-allocated capacity buffer (see above),
                // but it might be filled with a garbage (let's hope it is not).
                unsafe {
                    fs_type.set_len(MAX_PATH + 1);
                }
                let str_end = fs_type.iter().position(|chr| *chr == 0x00).unwrap_or(0);
                fs_type.truncate(str_end);

                let fs_type_str = OsString::from_wide(&fs_type);
                let fs = FileSystem::from_str(&fs_type_str.to_string_lossy())?;

                Ok(Some((drive_type, flags, fs)))
            }
        }
    }
}

impl<T> From<T> for Drive
where
    T: AsRef<[u16]>,
{
    fn from(data: T) -> Drive {
        let buffer = data.as_ref();
        debug_assert!(buffer.len() == 4);
        debug_assert!(buffer[0] >= 0x0041 && buffer[0] <= 0x005a);
        debug_assert!(buffer[1] == 0x003a);
        debug_assert!(buffer[2] == 0x005c);
        debug_assert!(buffer[3] == 0x0000);

        let mut inner = [0; 4];
        inner.copy_from_slice(buffer);
        Drive(inner)
    }
}
