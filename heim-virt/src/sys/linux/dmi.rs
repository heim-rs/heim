use heim_common::prelude::{
    future, stream, Future, FutureExt, StreamExt, TryFutureExt, TryStreamExt,
};
use heim_runtime::fs;

use crate::Virtualization;

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
))]
pub fn detect_vm_dmi() -> impl Future<Output = Result<Virtualization, ()>> {
    stream::iter(&[
        "/sys/class/dmi/id/product_name", /* Test this before sys_vendor to detect KVM over QEMU */
        "/sys/class/dmi/id/sys_vendor",
        "/sys/class/dmi/id/board_vendor",
        "/sys/class/dmi/id/bios_vendor",
    ])
    .then(|path| fs::read_first_line(path).into_future())
    .map_err(|_| ())
    .try_filter_map(|line| match () {
        _ if line.starts_with("KVM") => future::ok(Some(Virtualization::Kvm)),
        _ if line.starts_with("QEMU") => future::ok(Some(Virtualization::Qemu)),
        _ if line.starts_with("VMware") => future::ok(Some(Virtualization::Vmware)),
        _ if line.starts_with("VMW") => future::ok(Some(Virtualization::Vmware)),
        _ if line.starts_with("innotek GmbH") => future::ok(Some(Virtualization::Oracle)),
        _ if line.starts_with("Xen") => future::ok(Some(Virtualization::Xen)),
        _ if line.starts_with("Bochs") => future::ok(Some(Virtualization::Bochs)),
        _ if line.starts_with("Parallels") => future::ok(Some(Virtualization::Parallels)),
        _ if line.starts_with("BHYVE") => future::ok(Some(Virtualization::Bhyve)),
        _ => future::ok(None),
    })
    .into_future()
    .map(|(res, _)| match res {
        Some(Ok(virt)) => Ok(virt),
        _ => Err(()),
    })
}

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
)))]
pub fn detect_vm_dmi() -> impl Future<Output = Result<Virtualization, ()>> {
    future::err(())
}
