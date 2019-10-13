#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use raw_cpuid::{CpuId, Hypervisor};

use crate::Virtualization;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn detect_vm_cpuid() -> Result<Virtualization, ()> {
    let cpuid = match CpuId::new().get_hypervisor_info() {
        Some(info) => info,
        None => return Err(()),
    };

    match cpuid.identify() {
        Hypervisor::Xen => Ok(Virtualization::Xen),
        // https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/reference/tlfs
        Hypervisor::HyperV => Ok(Virtualization::HyperV),
        Hypervisor::KVM => Ok(Virtualization::Kvm),
        // https://kb.vmware.com/s/article/1009458
        Hypervisor::VMware => Ok(Virtualization::Vmware),
        // `TCGTCGTCGTCG`
        Hypervisor::Unknown(0x5447_4354, 0x4354_4743, 0x4743_5447) => Ok(Virtualization::Qemu),
        // `bhyve bhyve `
        Hypervisor::Unknown(0x7679_6862, 0x6862_2065, 0x2065_7679) => Ok(Virtualization::Bhyve),
        // `QNXQVMBSQG`
        Hypervisor::Unknown(0x5158_4e51, 0x5342_4d56, 0x0000_4751) => Ok(Virtualization::Qnx),
        // ACRNACRNACRN
        Hypervisor::Unknown(0x4e52_4341, 0x4e52_4341, 0x4e52_4341) => Ok(Virtualization::Acrn),
        Hypervisor::Unknown(b, c, d) => {
            dbg!((b, c, d));
            Err(())
        }
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn detect_vm_cpuid() -> Result<Virtualization, ()> {
    Err(())
}
