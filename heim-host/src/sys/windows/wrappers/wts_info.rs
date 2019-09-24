use std::fmt;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::shared::ntdef::{PVOID, WCHAR};

use super::super::bindings::wtsapi32;

pub struct WtsInfo(pub wtsapi32::WTSINFOW);

impl WtsInfo {
    pub fn username(&self) -> Option<String> {
        if self.0.UserName[0] == 0x00 {
            None
        } else {
            Some(Self::from_wide(&self.0.UserName))
        }
    }

    pub fn domain(&self) -> String {
        Self::from_wide(&self.0.UserName)
    }

    // TODO: Seems like it is used widely across `heim`, should be refactored
    fn from_wide(chars: &[WCHAR]) -> String {
        // TODO: Use `memchr` crate if possible?
        let first_null = chars.iter().position(|c| *c == 0x00).unwrap_or(0);
        OsString::from_wide(&chars[..first_null])
            .to_string_lossy()
            .to_string()
    }
}

impl Drop for WtsInfo {
    #[allow(trivial_casts)]
    fn drop(&mut self) {
        unsafe {
            wtsapi32::WTSFreeMemory(&mut self.0 as *mut _ as PVOID);
        }
    }
}

impl fmt::Debug for WtsInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WtsInfo")
            .field("State", &self.0.State)
            .field("SessionId", &self.0.SessionId)
            .field("IncomingBytes", &self.0.IncomingBytes)
            .field("OutgoingBytes", &self.0.OutgoingBytes)
            .field("IncomingFrames", &self.0.IncomingFrames)
            .field("OutgoingFrames", &self.0.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.0.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.0.OutgoingCompressedBytes)
            .field("WinStationName", &self.0.WinStationName)
            .field("Domain", &self.0.Domain)
            .field("UserName", &self.0.UserName)
            .field("ConnectTime", unsafe { &self.0.ConnectTime.QuadPart() })
            .field("DisconnectTime", unsafe {
                &self.0.DisconnectTime.QuadPart()
            })
            .field("LastInputTime", unsafe { &self.0.LastInputTime.QuadPart() })
            .field("LogonTime", unsafe { &self.0.LogonTime.QuadPart() })
            .field("CurrentTime", unsafe { &self.0.CurrentTime.QuadPart() })
            .finish()
    }
}
