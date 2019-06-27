use heim_common::prelude::*;

use crate::Address;

#[derive(Debug)]
pub struct Nic;

impl Nic {
    pub fn name(&self) -> &str {
        unimplemented!()
    }

    pub fn address(&self) -> Address {
        unimplemented!()
    }

    pub fn netmask(&self) -> Option<Address> {
        unimplemented!()
    }

    pub fn broadcast(&self) -> Option<Address> {
        unimplemented!()
    }

    pub fn destination(&self) -> Option<Address> {
        unimplemented!()
    }

    pub fn is_up(&self) -> bool {
        unimplemented!()
    }

    pub fn is_broadcast(&self) -> bool {
        unimplemented!()
    }

    pub fn is_loopback(&self) -> bool {
        unimplemented!()
    }

    pub fn is_point_to_point(&self) -> bool {
        unimplemented!()
    }

    pub fn is_multicast(&self) -> bool {
        unimplemented!()
    }
}

pub fn nic() -> impl Stream<Item = Result<Nic>> {
    // TODO: Stub
    stream::iter(vec![])
}
