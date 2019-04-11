use std::fmt;

use crate::{sys, Inode};

#[derive(heim_derive::ImplWrap)]
pub struct UnixConnection(sys::UnixConnection);

impl UnixConnection {
    /// Returns socket inode number.
    pub fn inode(&self) -> Inode {
        self.as_ref().inode()
    }

    /// Returns pathname to which the socket was bound.
    pub fn source(&self) -> Option<&str> {
        self.as_ref().source()
    }

    /// Returns peer's inode number if socket is connected.
    pub fn peer(&self) -> Option<Inode> {
        self.as_ref().peer()
    }
}

impl fmt::Debug for UnixConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnixConnection")
            .field("inode", &self.inode())
            .field("source", &self.source())
            .field("peer", &self.peer())
            .finish()
    }
}
