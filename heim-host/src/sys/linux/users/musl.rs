use std::{convert::TryFrom, net::IpAddr};

use heim_common::prelude::*;
use heim_common::{Pid, Uid};

use crate::os::linux::SessionId;

#[derive(Debug)]
pub struct User;

impl User {
    pub fn username(&self) -> &str {
        unimplemented!("https://github.com/heim-rs/heim/issues/141")
    }

    pub fn terminal(&self) -> &str {
        unimplemented!("https://github.com/heim-rs/heim/issues/141")
    }

    pub fn id(&self) -> &str {
        unimplemented!("https://github.com/heim-rs/heim/issues/141")
    }

    pub fn pid(&self) -> Pid {
        unimplemented!("https://github.com/heim-rs/heim/issues/141")
    }

    pub fn hostname(&self) -> &str {
        unimplemented!("https://github.com/heim-rs/heim/issues/141")
    }

    pub fn address(&self) -> Option<IpAddr> {
        unimplemented!("https://github.com/heim-rs/heim/issues/141")
    }

    pub fn session_id(&self) -> SessionId {
        unimplemented!("https://github.com/heim-rs/heim/issues/141")
    }
}

pub async fn users() -> Result<impl Stream<Item = Result<User>>> {
    Ok(stream::empty())
}

impl TryFrom<Uid> for User {
    type Error = Error;
    fn try_from(_uid: Uid) -> Result<Self> {
        unimplemented!("https://github.com/heim-rs/heim/issues/141")
    }
}
