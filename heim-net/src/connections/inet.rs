use std::fmt;
use std::net::SocketAddr;

use crate::{sys, TcpState};

/// Inet (`AF_INET`) connection.
#[derive(heim_derive::ImplWrap)]
pub struct InetConnection(sys::InetConnection);

impl InetConnection {
    /// Returns connection source address if available.
    pub fn source(&self) -> Option<&SocketAddr> {
        self.as_ref().source()
    }

    /// Returns connection destination address is available.
    pub fn destination(&self) -> Option<&SocketAddr> {
        self.as_ref().destination()
    }

    pub fn state(&self) -> TcpState {
        self.as_ref().state()
    }
}

impl fmt::Debug for InetConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InetConnection")
            .field("source", &self.source())
            .field("destination", &self.destination())
            .field("state", &self.state())
            .finish()
    }
}
