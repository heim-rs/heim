use std::fmt;
use std::net;

use heim_common::prelude::*;

use crate::sys;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Address {
    Inet(net::SocketAddr),
    // TODO: Create and store LinkAddr here
    Link,
    #[doc(hidden)]
    __Nonexhaustive,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AddressFamily {
    /// Local communication (see `unix(7)`)
    Unix,
    /// IPv4 Internet protocols (see `ip(7)`)
    Inet,
    /// IPv6 Internet protocols (see `ipv6(7)`)
    Inet6,
    /// Low level packet interface (see `packet(7)`)
    Packet,
    Link, // macaddr
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Network interface device.
#[derive(heim_derive::ImplWrap)]
pub struct Nic(sys::Nic);

impl Nic {
    /// Returns NIC name.
    pub fn name(&self) -> &str {
        self.as_ref().name()
    }

    /// Returns NIC family.
    pub fn family(&self) -> AddressFamily {
        self.as_ref().family()
    }

    pub fn address(&self) -> Address {
        self.as_ref().address()
    }

    pub fn netmask(&self) -> Option<Address> {
        self.as_ref().netmask()
    }

    pub fn broadcast(&self) -> Option<Address> {
        self.as_ref().broadcast()
    }

    pub fn destination(&self) -> Option<Address> {
        self.as_ref().destination()
    }

    pub fn is_up(&self) -> bool {
        self.as_ref().is_up()
    }

    pub fn is_broadcast(&self) -> bool {
        self.as_ref().is_broadcast()
    }

    pub fn is_loopback(&self) -> bool {
        self.as_ref().is_loopback()
    }

    pub fn is_point_to_point(&self) -> bool {
        self.as_ref().is_point_to_point()
    }

    pub fn is_multicast(&self) -> bool {
        self.as_ref().is_multicast()
    }
}

impl fmt::Debug for Nic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Nic")
            .field("name", &self.name())
            .field("family", &self.family())
            .field("address", &self.address())
            .field("netmask", &self.netmask())
            .field("broadcast", &self.broadcast())
            .field("destination", &self.destination())
            .field("is_up", &self.is_up())
            .field("is_broadcast", &self.is_broadcast())
            .field("is_loopback", &self.is_loopback())
            .field("is_point_to_point", &self.is_point_to_point())
            .field("is_multicast", &self.is_multicast())
            .finish()
    }
}

/// Returns stream which yields [Network Interface Cards].
///
/// [Network Interface Cards]: struct.Nic.html
pub fn nic() -> impl Stream<Item = Nic, Error = Error> {
    sys::nic().map(Into::into)
}
