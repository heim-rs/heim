use std::io;
use std::ptr;
use std::str::FromStr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::ctypes::wchar_t;
use winapi::shared::minwindef::{DWORD, MAX_PATH};
use winapi::um::{fileapi, winbase, errhandlingapi};
use heim_common::{Result, Error};

use crate::FileSystem;
use crate::os::windows::{DriveType, Flags};

#[derive(Debug)]
pub struct LogicalDrive(pub Vec<wchar_t>);

impl LogicalDrive {
    pub fn to_os_string(&self) -> OsString {
        OsString::from_wide(&self.0)
    }

    pub fn volume_name(&self) -> Result<OsString> {
        let mut volume: Vec<wchar_t> = Vec::with_capacity(50);
        let result = unsafe {
            fileapi::GetVolumeNameForVolumeMountPointW(
                self.0.as_ptr(),
                volume.as_mut_ptr(),
                50,
            )
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            unsafe {
                volume.set_len(50);
            }

            let str_end = volume.iter().position(|chr| *chr == 0x00).unwrap_or(0);
            volume.truncate(str_end);
            let volume_path = OsString::from_wide(&volume);

            Ok(volume_path)
        }
    }

    pub fn drive_type(&self) -> Option<DriveType> {
        let result = unsafe {
            fileapi::GetDriveTypeW(self.0.as_ptr())
        };

        DriveType::maybe_from(result)
    }

    pub fn information(&self) -> Result<Option<(Flags, FileSystem)>> {
        let mut flags: DWORD = 0;
        let mut fs_type: Vec<wchar_t> = Vec::with_capacity(MAX_PATH + 1);
        let old_mode = unsafe {
            errhandlingapi::SetErrorMode(winbase::SEM_FAILCRITICALERRORS)
        };

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

        let _ = unsafe {
            errhandlingapi::SetErrorMode(old_mode)
        };

        if result == 0 {
            let e = io::Error::last_os_error();
            match e.raw_os_error() {
                // With floppy and CD drives we might get some errors if they are empty.
                // We are going to ignore them at all in these cases
                // TODO: Is there maybe some constant instead of these magical `21` and `123`?
                // errno 21 - "Device not ready"
                //      when there is no floppy or disk
                //
                // errno 123 - The filename, directory name, or volume label syntax is incorrect.
                //      happens at Azure Pipelines, probably should be ignored too
                Some(0) | Some(21) | Some(123) => Ok(None),
                _ => Err(e.into()),
            }
        } else {
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

            Ok(Some((flags, fs)))
        }
    }
}

/// Iterator over valid drives in system.
pub struct LogicalDrives {
    buffer: Vec<wchar_t>,
    position: DWORD,
}

impl LogicalDrives {
    pub fn new() -> Result<LogicalDrives> {
        let expected_size = unsafe {
            fileapi::GetLogicalDriveStringsW(
                0,
                ptr::null_mut(),
            )
        };

        if expected_size == 0 {
            return Err(Error::last_os_error());
        }

        let mut buffer = Vec::with_capacity(expected_size as usize);
        let result = unsafe {
            fileapi::GetLogicalDriveStringsW(
                expected_size,
                buffer.as_mut_ptr()
            )
        };

        if result == 0 {
            return Err(Error::last_os_error());
        }

        // the return value is the length, in characters, of the strings copied to the buffer,
        // not including the terminating null character.
        debug_assert!(expected_size == result + 1);
        unsafe {
            buffer.set_len((result + 1) as usize);
        }

        Ok(LogicalDrives {
            buffer,
            position: 0,
        })
    }
}

impl Iterator for LogicalDrives {
    type Item = LogicalDrive;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position as usize;

        match self.buffer.iter().skip(position).position(|chr| *chr == 0x00) {
            Some(end) if end > 0 => {
                let drive = Vec::from(&self.buffer[position..position + end]);
                self.position = (position + end + 1) as DWORD;
                Some(LogicalDrive(drive))
            },
            _ => None,
        }
    }
}

impl DriveType {
    #[doc(hidden)]
    fn maybe_from(value: DWORD) -> Option<DriveType> {
        match value {
            winbase::DRIVE_CDROM => Some(DriveType::CdRom),
            winbase::DRIVE_FIXED => Some(DriveType::Fixed),
            winbase::DRIVE_NO_ROOT_DIR => Some(DriveType::NoRootDir),
            winbase::DRIVE_RAMDISK => Some(DriveType::RamDisk),
            winbase::DRIVE_REMOTE => Some(DriveType::Remote),
            winbase::DRIVE_REMOVABLE => Some(DriveType::Removable),
            winbase::DRIVE_UNKNOWN => None,
            other => unreachable!("Unknown drive type {}", other),
        }
    }
}
