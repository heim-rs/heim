use std::net::IpAddr;

use heim_common::prelude::*;
use super::bindings;

#[derive(Debug)]
pub struct User {
    domain: String,
    username: String,
    address: Option<IpAddr>,
}

impl User {
    pub fn from_session(session: bindings::Session) -> Result<Option<User>> {
        let info = session.info()?;
        let address = session.address()?;

        // Fast-skipping users with empty username
        match info.UserName.iter().next() {
            Some(0x00) | None => return Ok(None),
            _ => {}
        }

        let username = bindings::Session::from_wide(&info.UserName);
        let domain = bindings::Session::from_wide(&info.Domain);

        Ok(Some(User {
            domain,
            username,
            address,
        }))
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


pub fn users() -> impl Stream<Item=Result<User>> {
    future::lazy(|_| {
        let sessions = bindings::Sessions::new()?;

        Ok(stream::iter(sessions).map(Ok))
    })
    .try_flatten_stream()
    .try_filter_map(|session| {
        future::ready(User::from_session(session))
    })
}
