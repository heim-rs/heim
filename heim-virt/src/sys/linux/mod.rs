use heim_common::prelude::{future, Future, FutureExt, TryFutureExt};

use crate::Virtualization;

mod containers;
mod cpuid;
mod device_tree;
mod dmi;

pub fn detect() -> impl Future<Output = Option<Virtualization>> {
    future::err(())
        .or_else(|_| self::containers::detect_container())
        .or_else(|_| self::dmi::detect_vm_dmi())
        .or_else(|_| future::ready(self::cpuid::detect_vm_cpuid()))
        .or_else(|_| self::device_tree::detect_vm_device_tree())
        .map(|res| res.ok())
}
