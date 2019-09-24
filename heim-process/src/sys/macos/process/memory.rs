use heim_common::units::{information, Information};

#[derive(Debug)]
pub struct Memory {
    pti_resident_size: Information,
    pti_virtual_size: Information,
    // number of page faults
    pti_faults: u64,
    // number of actual pageins
    pti_pageins: u64,
}

impl Memory {
    pub fn rss(&self) -> Information {
        self.pti_resident_size
    }

    pub fn vms(&self) -> Information {
        self.pti_virtual_size
    }

    pub fn faults(&self) -> u64 {
        self.pti_faults
    }

    pub fn pageins(&self) -> u64 {
        self.pti_pageins
    }
}

impl From<darwin_libproc::proc_taskinfo> for Memory {
    fn from(info: darwin_libproc::proc_taskinfo) -> Memory {
        Memory {
            pti_resident_size: Information::new::<information::byte>(info.pti_resident_size),
            pti_virtual_size: Information::new::<information::byte>(info.pti_virtual_size),
            // TODO: Is it reasonable to convert into `u64`?
            pti_faults: info.pti_faults as u64,
            pti_pageins: info.pti_pageins as u64,
        }
    }
}
