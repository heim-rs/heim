use heim_common::prelude::*;

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;

use std::ffi::CStr;

use winapi::shared::minwindef::ULONG;
use winapi::shared::ntdef::NULL;
use winapi::shared::ws2def::SOCKET_ADDRESS;
use winapi::shared::ws2ipdef::SOCKADDR_IN6;
use winapi::shared::ws2def::{AF_INET, AF_INET6};
use winapi::shared::ws2def::AF_UNSPEC;
use winapi::shared::winerror::{ERROR_BUFFER_OVERFLOW, NO_ERROR};
use winapi::um::iphlpapi::GetAdaptersAddresses;
use winapi::um::iptypes::GAA_FLAG_INCLUDE_PREFIX;
use winapi::um::iptypes::PIP_ADAPTER_ADDRESSES;
use winapi::um::iptypes::IP_ADAPTER_ADDRESSES;

use crate::Address;



#[derive(Debug)]
pub struct Nic {
    index: u32,
    name: String,
    address: Option<Address>,
    netmask: Option<Address>,
}


fn sockaddr_to_ipv4(sa: SOCKET_ADDRESS) -> Option<Address> {
    // Check this sockaddr can fit one short and a char[14]
    // (see https://docs.microsoft.com/en-us/windows/win32/winsock/sockaddr-2)
    // This should always happen though, this is guaranteed by winapi's interface
    if sa.iSockaddrLength < 2+14 {
        return None;
    }

    if sa.lpSockaddr.is_null() {
        return None;
    }
    let arr = unsafe { (*sa.lpSockaddr).sa_data };
    let ip4 = Ipv4Addr::new(arr[2] as _, arr[3] as _, arr[4] as _, arr[5] as _);
    let port = (arr[0] as u16) + (arr[1] as u16)*0x100;

    Some(Address::Inet(
        SocketAddrV4::new(ip4, port)
    ))
}

fn sockaddr_to_ipv6(sa: SOCKET_ADDRESS) -> Option<Address> {
    // Check this sockaddr can fit a SOCKADDR_IN6 (two shorts, two longs, and a 16-byte struct)
    // (see https://docs.microsoft.com/en-us/windows/win32/winsock/sockaddr-2)
    if (sa.iSockaddrLength as usize) < std::mem::size_of::<SOCKADDR_IN6>() {
        return None;
    }

    let p_sa6 = sa.lpSockaddr as *const SOCKADDR_IN6;
    if p_sa6.is_null() {
        return None;
    }
    let sa6 = unsafe{ *p_sa6 };

    let ip6_data = unsafe{ sa6.sin6_addr.u.Byte() };
    let ip6 = Ipv6Addr::from(*ip6_data);
    let port = sa6.sin6_port;
    let flow_info = sa6.sin6_flowinfo;
    let scope_id = unsafe{ *sa6.u.sin6_scope_id() };

    Some(Address::Inet6(
        SocketAddrV6::new(ip6, port, flow_info, scope_id )
    ))
}

/// Generate an IPv4 netmask from a prefix length (Rust equivalent of ConvertLengthToIpv4Mask())
fn ipv4_netmask_from(length: u8) -> Ipv4Addr {
    let mask = match length {
        len if len <= 32 => u32::max_value().checked_shl(32 - len as u32).unwrap_or(0),
        _ /* invalid value */ => u32::max_value(),
    };
    Ipv4Addr::from(mask)
}

/// Generate an IPv6 netmask from a prefix length
fn ipv6_netmask_from(length: u8) -> Ipv6Addr {
    let mask = match length {
        len if len <= 128 => u128::max_value().checked_shl(128 - len as u32).unwrap_or(0),
        _ /* invalid value */ => u128::max_value(),
    };
    Ipv6Addr::from(mask)
}

fn ipv4_netmask_address_from(length: u8) -> Address {
    Address::Inet(SocketAddrV4::new(ipv4_netmask_from(length), 0))
}
fn ipv6_netmask_address_from(length: u8) -> Address {
    Address::Inet6(SocketAddrV6::new(ipv6_netmask_from(length), 0, 0, 0))
}





impl Nic {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index(&self) -> Option<u32> {
        Some(self.index)
    }

    pub fn address(&self) -> Address {
        self.address.unwrap_or_else(||
            Address::Inet(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0), 0))
        )
    }

    pub fn netmask(&self) -> Option<Address> {
        self.netmask
    }

    pub fn destination(&self) -> Option<Address> {
        // TODO: we could implement something one day
        None
    }

    pub fn is_up(&self) -> bool {
        // TODO: not sure how to tell on Windows
        true
    }

    pub fn is_running(&self) -> bool {
        // TODO: not sure how to tell on Windows
        true
    }

    pub fn is_loopback(&self) -> bool {
        match self.address {
            Some(Address::Inet(sa)) => sa.ip().is_loopback(),
            Some(Address::Inet6(sa6)) => sa6.ip().is_loopback(),
            _ => false
        }
    }

    pub fn is_multicast(&self) -> bool {
        match self.address {
            Some(Address::Inet(sa)) => sa.ip().is_multicast(),
            Some(Address::Inet6(sa6)) => sa6.ip().is_multicast(),
            _ => false
        }
    }
}

pub async fn nic() -> Result<impl Stream<Item = Result<Nic>> + Send + Sync> {
    let mut results = Vec::new();

    // Step 1 - get the size of the routing infos
    let family = AF_UNSPEC; // retrieve both IPv4 and IPv6 interfaces
    let flags: ULONG = GAA_FLAG_INCLUDE_PREFIX;
    let mut empty_list = IP_ADAPTER_ADDRESSES::default();
    let mut data_size: ULONG = 0;
    let res = unsafe { GetAdaptersAddresses(family as _, flags, NULL, &mut empty_list, &mut data_size) };
    if res != ERROR_BUFFER_OVERFLOW {
        // Unable to get the size of routing infos
        let e = Error::from(std::io::Error::from_raw_os_error(res as _)).with_ffi("GetAdaptersAddresses");
        return Err(e);
    }


    // Step 2 - get the interfaces infos
    let mut buffer = vec![0; data_size as usize];
    let res = unsafe { GetAdaptersAddresses(family as _, flags, NULL, buffer.as_mut_ptr() as _, &mut data_size) };
    if res != NO_ERROR {
        // Unable to get the routing infos
        let e = Error::from(std::io::Error::from_raw_os_error(res as _)).with_ffi("GetAdaptersAddresses");
        return Err(e);
    }


    // Step 3 - walk through the list and populate our interfaces
    let mut cur_iface = unsafe {
        let p = buffer.as_ptr() as PIP_ADAPTER_ADDRESSES;
        if p.is_null() {
            // Unable to list interfaces
            let e = Error::from(std::io::Error::from_raw_os_error(res as _)).with_ffi("GetAdaptersAddresses");
            return Err(e);
        }
        *p
    };

    loop {
        let iface_index;
        let iface_name_cstr;
        let mut cur_address;

        unsafe {
            iface_index = cur_iface.u.s().IfIndex;
            iface_name_cstr = CStr::from_ptr(cur_iface.AdapterName);
            cur_address = *(cur_iface.FirstUnicastAddress);
        }
        let iface_name = iface_name_cstr.to_str().map(|s| s.to_string()).unwrap_or_else(|_| "".into());


        // Walk through every IP address of this interface
        let mut best_iface_address = None;
        let mut best_netmask_length = None;
        loop {
            let this_socket_address = cur_address.Address;
            let this_netmask_length = cur_address.OnLinkPrefixLength;
            let this_sa_family = unsafe { (*this_socket_address.lpSockaddr).sa_family };

            let this_address = match this_sa_family as i32 {
                AF_INET => {
                    sockaddr_to_ipv4(this_socket_address)
                },
                AF_INET6 => {
                    sockaddr_to_ipv6(this_socket_address)
                },
                _ => None,
            };

            // heim::Nic only stores a "primary" address/mask.
            // We only keep an address/mask if we consider it more relevant than the others
            update_if_more_relevant(&mut best_iface_address, &mut best_netmask_length, this_address, this_netmask_length);

            let next_address = cur_address.Next;
            if next_address.is_null() {
                break;
            }
            cur_address = unsafe { *next_address };
        }


        // Build a netmask suited to the best address
        let iface_netmask = best_netmask_length.and_then(|len|
            match best_iface_address {
                Some(Address::Inet(_)) =>  Some(ipv4_netmask_address_from(len)),
                Some(Address::Inet6(_)) => Some(ipv6_netmask_address_from(len)),
                _ => None,
            }
        );


        results.push(Ok(Nic{
            index: iface_index,
            name: iface_name,
            address: best_iface_address,
            netmask: iface_netmask,
        }));

        let next_item = cur_iface.Next;
        if next_item.is_null() {
            break;
        }
        cur_iface = unsafe { *next_item };

    }

    Ok(stream::iter(results))
}


fn update_if_more_relevant(best_iface_address: &mut Option<Address>, best_netmask_length: &mut Option<u8>, this_address: Option<Address>, this_netmask_length: u8) {
    if let Some(a) = this_address {
        if best_iface_address.is_none() {
            // Any address is more relevant than no address
            *best_iface_address = Some(a);
            *best_netmask_length = Some(this_netmask_length);

        }else{
            match a {
                Address::Inet6(addr6) => {
                    if (addr6.ip().segments()[0] & 0xffc0) != 0xfe80 {
                        // it is not link-local
                        *best_iface_address = Some(a);
                        *best_netmask_length = Some(this_netmask_length);

                    }
                },
                Address::Inet(addr4) => {
                    if ! addr4.ip().is_link_local() {
                        *best_iface_address = Some(a);
                        *best_netmask_length = Some(this_netmask_length);

                    }
                },
                _ => (),
            }
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_netmasks() {
        assert_eq!(ipv4_netmask_from(0),   Ipv4Addr::new(0,0,0,0));
        assert_eq!(ipv4_netmask_from(32),  Ipv4Addr::new(255,255,255,255));
        assert_eq!(ipv4_netmask_from(200), Ipv4Addr::new(255,255,255,255));
        assert_eq!(ipv4_netmask_from(9),   Ipv4Addr::new(255,128,0,0));

        assert_eq!(ipv6_netmask_from(0),   Ipv6Addr::new(0,0,0,0,0,0,0,0));
        assert_eq!(ipv6_netmask_from(32),  Ipv6Addr::new(0xffff, 0xffff, 0, 0, 0, 0, 0, 0));
        assert_eq!(ipv6_netmask_from(200), Ipv6Addr::new(0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff));
        assert_eq!(ipv6_netmask_from(9),   Ipv6Addr::new(0xff80, 0, 0, 0, 0, 0, 0, 0));
    }
}
