use std::fmt;

use heim_common::prelude::*;
use heim_common::units::{information, Information};

use super::bindings::{if_msghdr2, net_pf_route};

pub struct IoCounters {
    name: String,
    data: if_msghdr2,
}

impl IoCounters {
    pub fn interface(&self) -> &str {
        self.name.as_str()
    }

    pub fn bytes_sent(&self) -> Information {
        Information::new::<information::byte>(self.data.ifm_data.ifi_obytes)
    }

    pub fn bytes_recv(&self) -> Information {
        Information::new::<information::byte>(self.data.ifm_data.ifi_ibytes)
    }

    pub fn packets_sent(&self) -> u64 {
        self.data.ifm_data.ifi_opackets
    }

    pub fn packets_recv(&self) -> u64 {
        self.data.ifm_data.ifi_ipackets
    }

    pub fn errors_sent(&self) -> u64 {
        self.data.ifm_data.ifi_oerrors
    }

    pub fn errors_recv(&self) -> u64 {
        self.data.ifm_data.ifi_ierrors
    }

    pub fn drop_recv(&self) -> u64 {
        self.data.ifm_data.ifi_iqdrops
    }
}

impl fmt::Debug for IoCounters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IoCounters")
            .field("name", &self.name)
            .finish()
    }
}

pub fn io_counters() -> impl Stream<Item = Result<IoCounters>> {
    future::lazy(|_| unsafe { net_pf_route() })
        .map_ok(|interfaces| stream::iter(interfaces).map(Ok))
        .try_flatten_stream()
        .and_then(|msg: if_msghdr2| {
            let mut name: [u8; libc::IF_NAMESIZE] = [0; libc::IF_NAMESIZE];
            let result = unsafe {
                libc::if_indextoname(msg.ifm_index.into(), name.as_mut_ptr() as *mut libc::c_char)
            };
            if result.is_null() {
                return future::err(Error::last_os_error());
            }
            let first_nul = name.iter().position(|c| *c == b'\0').unwrap_or(0);
            let name = String::from_utf8_lossy(&name[..first_nul]).to_string();

            future::ok(IoCounters { name, data: msg })
        })
}
