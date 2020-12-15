use std::fmt;
use std::net;

use heim_common::prelude::*;

use crate::sys;

/// Network interface address.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Address {
    /// IPv4 Internet protocols
    Inet(net::SocketAddrV4),

    /// IPv6 Internet protocols
    Inet6(net::SocketAddrV6),

    /// Link level interface
    Link(macaddr::MacAddr),
}

// TODO: Consider implement `Address::to_family()` method
// which will return the `libc::c_int` value with a corresponding value
// for the current address member (ex. `AF_INET` or `AF_PACKET`)
// Do not forget that it is OS-dependant.

/// Network interface device.
pub struct Nic(sys::Nic);

wrap!(Nic, sys::Nic);

impl Nic {
    /// Returns NIC name.
    pub fn name(&self) -> &str {
        self.as_ref().name()
    }

    /// Returns NIC index (internally used by the OS to identify the NIC)
    pub fn index(&self) -> Option<u32> {
        self.as_ref().index()
    }

    /// Returns primary NIC address.
    ///
    /// See [`nic`] for more info regarding NICs that have multiple addresses
    pub fn address(&self) -> Address {
        self.as_ref().address()
    }

    /// Returns netmask address if available.
    pub fn netmask(&self) -> Option<Address> {
        self.as_ref().netmask()
    }

    /// Returns destination address if available.
    pub fn destination(&self) -> Option<Address> {
        self.as_ref().destination()
    }

    /// Returns `bool` indicating whether interface is up.
    pub fn is_up(&self) -> bool {
        self.as_ref().is_up()
    }

    /// Returns `bool` indicating whether interface is running.
    pub fn is_running(&self) -> bool {
        self.as_ref().is_running()
    }

    /// Returns `bool` indicating whether interface is loopback.
    pub fn is_loopback(&self) -> bool {
        self.as_ref().is_loopback()
    }

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
            .field("destination", &self.destination())
            .field("is_up", &self.is_up())
            .field("is_loopback", &self.is_loopback())
            .field("is_multicast", &self.is_multicast())
            .finish()
    }
}

/// Returns a stream over the [Network Interface Cards].
///
/// [Network Interface Cards]: struct.Nic.html
///
/// Depending on your platform, NICs that have multiple addresses may be enumerated several times, with a different [`address`](Nic::address) every time.
pub async fn nic() -> Result<impl Stream<Item = Result<Nic>> + Send + Sync> {
    let inner = sys::nic().await?;

    Ok(inner.map_ok(Into::into))
}
