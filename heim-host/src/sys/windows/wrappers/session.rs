use std::net::{IpAddr, Ipv4Addr};
use std::ptr;

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{LPWSTR, PVOID};
use winapi::shared::ws2def::{AF_INET, AF_INET6, AF_IPX, AF_NETBIOS, AF_UNSPEC};

use heim_common::prelude::{Error, Result};

use super::super::bindings::wtsapi32;
use super::WtsInfo;

#[derive(Debug)]
pub struct Session {
    session_id: DWORD,
}

impl Session {
    pub fn new(session_id: DWORD) -> Session {
        Session { session_id }
    }

    // https://docs.microsoft.com/ru-ru/windows/desktop/api/wtsapi32/ns-wtsapi32-_wtsinfow
    #[allow(trivial_casts)]
    pub fn info(&self) -> Result<WtsInfo> {
        let mut buffer: wtsapi32::PWTSINFOW = ptr::null_mut();
        let mut bytes: DWORD = 0;
        let result = unsafe {
            wtsapi32::WTSQuerySessionInformationW(
                wtsapi32::WTS_CURRENT_SERVER_HANDLE,
                self.session_id,
                wtsapi32::WTSSessionInfo,
                &mut buffer as *mut wtsapi32::PWTSINFOW as *mut LPWSTR,
                &mut bytes,
            )
        };

        if result == 0 {
            return Err(Error::last_os_error());
        }

        unsafe { Ok(WtsInfo(*buffer)) }
    }

    #[allow(trivial_casts)]
    pub fn address(&self) -> Result<Option<IpAddr>> {
        let mut address_ptr: wtsapi32::PWTS_CLIENT_ADDRESS = ptr::null_mut();
        let mut address_bytes: DWORD = 0;
        let result = unsafe {
            wtsapi32::WTSQuerySessionInformationW(
                wtsapi32::WTS_CURRENT_SERVER_HANDLE,
                self.session_id,
                wtsapi32::WTSClientAddress,
                &mut address_ptr as *mut _ as *mut LPWSTR,
                &mut address_bytes,
            )
        };

        if result == 0 {
            return Err(Error::last_os_error());
        }

        let address = match unsafe { (*address_ptr).AddressFamily as i32 } {
            AF_INET => {
                let bytes = unsafe { (*address_ptr).Address };
                Some(IpAddr::V4(Ipv4Addr::new(
                    bytes[2], bytes[3], bytes[4], bytes[5],
                )))
            }
            AF_INET6 => {
                let bytes = unsafe { (*address_ptr).Address };
                let mut addr: [u8; 16] = [0; 16];
                addr.copy_from_slice(&bytes[2..18]);

                Some(IpAddr::from(addr))
            }

            // TODO: Implement address parsing for the following families
            // See `crate::os::windows::UserExt::address` comments additionally
            AF_IPX => None,
            AF_NETBIOS => None,
            AF_UNSPEC => None,

            other => unreachable!("Unknown family {}", other),
        };

        unsafe {
            wtsapi32::WTSFreeMemory(address_ptr as *mut _ as PVOID);
        }

        Ok(address)
    }
}
