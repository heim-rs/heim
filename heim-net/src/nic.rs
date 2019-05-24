use std::fmt;
use std::net;

use heim_common::prelude::*;

use crate::sys;

/// Network interface address.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Address {
    /// IPv4 Internet protocols
    Inet(net::SocketAddr),

    /// IPv6 Internet protocols
    Inet6(net::SocketAddr),

    /// Link level interface
    Link(macaddr::MacAddr),

    #[doc(hidden)]
    __Nonexhaustive,
}

// TODO: Consider implement `Address::to_family()` method
// which will return the `libc::c_int` value with a corresponding value
// for the current address member (ex. `AF_INET` or `AF_PACKET`)
// Do not forget that it is OS-dependant.

/// Network interface device.
#[derive(heim_derive::ImplWrap)]
pub struct Nic(sys::Nic);

impl Nic {
    /// Returns NIC name.
    pub fn name(&self) -> &str {
        self.as_ref().name()
    }

    /// Returns primary NIC address.
    pub fn address(&self) -> Address {
        self.as_ref().address()
    }

    /// Returns netmask address if available.
    pub fn netmask(&self) -> Option<Address> {
        self.as_ref().netmask()
    }

// TODO: Should be moved into the *Ext trait (since it is not available for Windows?)
//    /// Returns broadcast address if available.
//    pub fn broadcast(&self) -> Option<Address> {
//        self.as_ref().broadcast()
//    }

    /// Returns destination address if available.
    pub fn destination(&self) -> Option<Address> {
        self.as_ref().destination()
    }

    /// Returns `bool` indicating whether interface is up and running.
    pub fn is_up(&self) -> bool {
        self.as_ref().is_up()
    }

// TODO: Should be moved into the *Ext trait (since it is not available for Windows?)
//    pub fn is_broadcast(&self) -> bool {
//        self.as_ref().is_broadcast()
//    }

    /// Returns `bool` indicating whether interface is loopback.
    pub fn is_loopback(&self) -> bool {
        self.as_ref().is_loopback()
    }

// TODO: Should be moved into the *Ext trait (since it is not available for Windows?)
//    pub fn is_point_to_point(&self) -> bool {
//        self.as_ref().is_point_to_point()
//    }

    /// Returns `bool` indicating whether interface is multicast.
    pub fn is_multicast(&self) -> bool {
        self.as_ref().is_multicast()
    }
}

impl fmt::Debug for Nic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Nic")
            .field("name", &self.name())
            .field("address", &self.address())
            .field("netmask", &self.netmask())
//            .field("broadcast", &self.broadcast())
            .field("destination", &self.destination())
            .field("is_up", &self.is_up())
//            .field("is_broadcast", &self.is_broadcast())
            .field("is_loopback", &self.is_loopback())
//            .field("is_point_to_point", &self.is_point_to_point())
            .field("is_multicast", &self.is_multicast())
            .finish()
    }
}

/// Returns stream which yields [Network Interface Cards].
///
/// [Network Interface Cards]: struct.Nic.html
pub fn nic() -> impl Stream<Item = Result<Nic>> {
    sys::nic().map_ok(Into::into)
}
