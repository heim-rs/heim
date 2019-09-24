use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use winapi::ctypes::wchar_t;
use winapi::shared::minwindef::DWORD;
use winapi::um::{fileapi, winbase};

use crate::os::windows::DriveType;

impl DriveType {
    pub(crate) fn from_path<T: AsRef<Path>>(path: T) -> Option<DriveType> {
        let mut bytes = path.as_ref().as_os_str().encode_wide().collect::<Vec<_>>();
        bytes.push(0x0000);

        Self::from_slice(&bytes)
    }

    pub(crate) fn from_slice(chars: &[wchar_t]) -> Option<DriveType> {
        debug_assert!(
            chars.last() == Some(&0x0000),
            "Path for GetDriveTypeW should be null-terminated"
        );

        let result = unsafe { fileapi::GetDriveTypeW(chars.as_ptr()) };

        Self::maybe_from(result)
    }

    pub(crate) fn maybe_from(value: DWORD) -> Option<DriveType> {
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
