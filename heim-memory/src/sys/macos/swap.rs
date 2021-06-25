use heim_common::prelude::*;
use heim_common::units::{information, Information};

use super::{bindings, PAGE_SIZE};

#[derive(Debug)]
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

#[allow(clippy::useless_conversion)]
pub async fn swap() -> Result<Swap> {
    let xsw_usage = bindings::vm_swapusage()?;
    let vm_stats = bindings::host_vm_info()?;
    let page_size = *PAGE_SIZE;

    let total = Information::new::<information::byte>(u64::from(xsw_usage.xsu_total));
    let used = Information::new::<information::byte>(u64::from(xsw_usage.xsu_used));
    let free = Information::new::<information::byte>(u64::from(xsw_usage.xsu_avail));
    let sin = Information::new::<information::byte>(u64::from(vm_stats.pageins) * page_size);
    let sout = Information::new::<information::byte>(u64::from(vm_stats.pageouts) * page_size);

    Ok(Swap {
        total,
        used,
        free,
        sin,
        sout,
    })
}
