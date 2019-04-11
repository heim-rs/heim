use sock_diag::packet;

use heim_common::prelude::*;

use crate::{Connection, Inode};

#[derive(Debug)]
pub struct UnixConnection(packet::UnixDiagResponse);

impl UnixConnection {
    pub fn inode(&self) -> Inode {
        self.0.inode.into()
    }

    pub fn source(&self) -> Option<&str> {
        self.0.name()
    }

    pub fn peer(&self) -> Option<Inode> {
        self.0.peer().map(Into::into)
    }
}

impl From<packet::UnixDiagResponse> for UnixConnection {
    fn from(r: packet::UnixDiagResponse) -> UnixConnection {
        Self(r)
    }
}
pub fn unix_connections(handle: &sock_diag::Handle) -> impl Stream<Item = Connection, Error = Error> + Send {
    handle
        .unix()
        .list()
        .with_show(packet::UnixShow::PEER)
        .execute()
        .map_err(|e| Box::new(e.compat()).into())
        .map(|conn| {
            let inner = UnixConnection(conn);

            Connection::Unix(inner.into())
        })
}
