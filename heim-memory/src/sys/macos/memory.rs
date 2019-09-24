use heim_common::prelude::*;
use heim_common::units::{information, Information};

use super::{bindings, PAGE_SIZE};

#[derive(Debug)]
pub struct Memory {
    total: Information,
    available: Information,
    used: Information,
    free: Information,
    active: Information,
    inactive: Information,
    wire: Information,
}

impl Memory {
    pub fn total(&self) -> Information {
        self.total
    }

    pub fn available(&self) -> Information {
        self.available
    }

    pub fn used(&self) -> Information {
        self.used
    }

    pub fn free(&self) -> Information {
        self.free
    }

    pub fn active(&self) -> Information {
        self.active
    }

    pub fn inactive(&self) -> Information {
        self.inactive
    }

    pub fn wire(&self) -> Information {
        self.wire
    }
}

pub fn memory() -> impl Future<Output = Result<Memory>> {
    future::lazy(|_| {
        let total = unsafe { bindings::hw_memsize()? };
        let vm_stats = unsafe { bindings::host_vm_info()? };
        let page_size = *PAGE_SIZE;

        let total = Information::new::<information::byte>(total);
        let available = Information::new::<information::byte>(
            u64::from(vm_stats.active_count + vm_stats.free_count) * page_size,
        );
        let free = Information::new::<information::byte>(
            u64::from(vm_stats.free_count - vm_stats.speculative_count) * page_size,
        );
        let used = Information::new::<information::byte>(
            u64::from(vm_stats.active_count + vm_stats.wire_count) * page_size,
        );
        let active =
            Information::new::<information::byte>(u64::from(vm_stats.active_count) * page_size);
        let inactive =
            Information::new::<information::byte>(u64::from(vm_stats.inactive_count) * page_size);
        let wire =
            Information::new::<information::byte>(u64::from(vm_stats.wire_count) * page_size);

        Ok(Memory {
            total,
            available,
            free,
            used,
            active,
            inactive,
            wire,
        })
    })
}
