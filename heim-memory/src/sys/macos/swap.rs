use heim_common::prelude::*;
use heim_common::units::iec::information::byte;
use heim_common::units::iec::usize::Information;

use super::{bindings, PAGE_SIZE};

pub struct Swap {
    total: Information,
    used: Information,
    free: Information,
    sin: Information,
    sout: Information,
}

impl Swap {
    pub fn total(&self) -> Information {
        self.total
    }

    pub fn used(&self) -> Information {
        self.used
    }

    pub fn free(&self) -> Information {
        self.free
    }

    pub fn sin(&self) -> Option<Information> {
        Some(self.sin)
    }

    pub fn sout(&self) -> Option<Information> {
        Some(self.sout)
    }
}

pub fn swap() -> impl Future<Item = Swap, Error = Error> {
    future::lazy(|| {
        let xsw_usage = unsafe { bindings::vm_swapusage()? };
        let vm_stats = unsafe { bindings::host_vm_info()? };
        let page_size = *PAGE_SIZE;

        let total = Information::new::<byte>(xsw_usage.xsu_total as usize);
        let used = Information::new::<byte>(xsw_usage.xsu_used as usize);
        let free = Information::new::<byte>(xsw_usage.xsu_avail as usize);
        let sin = Information::new::<byte>(
            vm_stats.pageins as usize * page_size,
        );
        let sout = Information::new::<byte>(
            vm_stats.pageouts as usize * page_size,
        );

        Ok(Swap {
            total,
            free,
            used,
            sin,
            sout,
        })
    })
}
