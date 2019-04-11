use heim_common::prelude::*;
use sock_diag::packet;

use crate::{Connection, TcpState};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct InetConnection(packet::InetDiagResponse);

impl InetConnection {
    pub fn source(&self) -> Option<&SocketAddr> {
        self.0.id.src.as_ref()
    }

    pub fn destination(&self) -> Option<&SocketAddr> {
        self.0.id.dst.as_ref()
    }

    pub fn state(&self) -> TcpState {
        self.0.state.into()
    }
}

impl From<packet::InetDiagResponse> for InetConnection {
    fn from(r: packet::InetDiagResponse) -> InetConnection {
        Self(r)
    }
}

pub fn inet_connections(
    handle: &sock_diag::Handle,
    family: libc::c_int,
    protocol: libc::c_int,
) -> impl Stream<Item = Connection, Error = Error> + Send {
    let type_ = (family.clone(), protocol.clone());
    handle
        .inet()
        .list(family as u8, protocol as u8)
        .execute()
        .map_err(|e| Box::new(e.compat()).into())
        .map(move |conn| {
            let inner = InetConnection(conn);

            match type_ {
                (libc::AF_INET, libc::IPPROTO_TCP) => Connection::Tcp4(inner.into()),
                (libc::AF_INET6, libc::IPPROTO_TCP) => Connection::Tcp6(inner.into()),
                (libc::AF_INET, libc::IPPROTO_UDP) => Connection::Udp4(inner.into()),
                (libc::AF_INET6, libc::IPPROTO_UDP) => Connection::Udp6(inner.into()),
                _ => unimplemented!(),
            }
        })
}
