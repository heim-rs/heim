use std::net::IpAddr;
use std::ptr;
use winapi::shared::minwindef::DWORD;
use winapi::um::winbase::LookupAccountSidW;
use winapi::um::winnt::{SidTypeUser, SID_AND_ATTRIBUTES, SID_NAME_USE, WCHAR};

use super::wrappers::{Session, Sessions};
use heim_common::prelude::*;

#[derive(Debug)]
pub struct User {
    domain: String,
    username: String,
    address: Option<IpAddr>,
}

impl User {
    pub fn from_session(session: Session) -> Result<Option<User>> {
        let info = session.info()?;

        let username = match info.username() {
            None => return Ok(None),
            Some(username) => username,
        };
        let domain = info.domain();

        Ok(Some(User {
            domain,
            username,
            address: session.address()?,
        }))
    }

    pub fn try_from_sid(sid: &SID_AND_ATTRIBUTES) -> Result<Self> {
        // name and domain cannot be longer than 256
        let mut name_cch: DWORD = 256;
        let mut name: Vec<WCHAR> = Vec::with_capacity(name_cch as usize);
        let mut domain_cch: DWORD = 256;
        let mut domain: Vec<WCHAR> = Vec::with_capacity(domain_cch as usize);
        let mut account_type: SID_NAME_USE = 0;

        let result = unsafe {
            LookupAccountSidW(
                ptr::null(),
                sid.Sid,
                name.as_mut_ptr(),
                &mut name_cch,
                domain.as_mut_ptr(),
                &mut domain_cch,
                &mut account_type,
            )
        };

        if result == 0 || account_type != SidTypeUser {
            return Err(Error::last_os_error().with_ffi("LookupAccountSidW"));
        }

        unsafe {
            name.set_len(name_cch as usize);
            domain.set_len(domain_cch as usize);
        }

        Ok(Self {
            domain: String::from_utf16(domain.as_slice())?,
            username: String::from_utf16(name.as_slice())?,
            address: None,
        })
    }

    pub fn domain(&self) -> &str {
        self.domain.as_str()
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn address(&self) -> Option<&IpAddr> {
        self.address.as_ref()
    }
}

pub fn users() -> impl Stream<Item = Result<User>> {
    future::lazy(|_| {
        let sessions = Sessions::new()?;

        Ok(stream::iter(sessions).map(Ok))
    })
    .try_flatten_stream()
    .try_filter_map(|session| future::ready(User::from_session(session)))
}
