use heim_common::units::Information;

// TODO: Stub: https://github.com/heim-rs/heim/issues/122
#[derive(Debug)]
pub struct Memory {}

impl Memory {
    pub fn rss(&self) -> Information {
        unimplemented!("https://github.com/heim-rs/heim/issues/122")
    }

    pub fn vms(&self) -> Information {
        unimplemented!("https://github.com/heim-rs/heim/issues/122")
    }
}
