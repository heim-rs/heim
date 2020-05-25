use heim_rt as rt;

use crate::Virtualization;

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
))]
pub async fn detect_vm_dmi() -> Result<Virtualization, ()> {
    const PROBE_FILES: [&str; 4] = [
        "/sys/class/dmi/id/product_name", /* Test this before sys_vendor to detect KVM over QEMU */
        "/sys/class/dmi/id/sys_vendor",
        "/sys/class/dmi/id/board_vendor",
        "/sys/class/dmi/id/bios_vendor",
    ];

    for filename in &PROBE_FILES {
        let line = match rt::fs::read_first_line(filename).await {
            Ok(line) => line,
            Err(..) => continue,
        };

        match () {
            _ if line.starts_with("KVM") => return Ok(Virtualization::Kvm),
            _ if line.starts_with("QEMU") => return Ok(Virtualization::Qemu),
            _ if line.starts_with("VMware") => return Ok(Virtualization::Vmware),
            _ if line.starts_with("VMW") => return Ok(Virtualization::Vmware),
            _ if line.starts_with("innotek GmbH") => return Ok(Virtualization::Oracle),
            _ if line.starts_with("Xen") => return Ok(Virtualization::Xen),
            _ if line.starts_with("Bochs") => return Ok(Virtualization::Bochs),
            _ if line.starts_with("Parallels") => return Ok(Virtualization::Parallels),
            _ if line.starts_with("BHYVE") => return Ok(Virtualization::Bhyve),
            _ => continue,
        }
    }

    Err(())
}

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
)))]
pub async fn detect_vm_dmi() -> Result<Virtualization, ()> {
    Err(())
}
