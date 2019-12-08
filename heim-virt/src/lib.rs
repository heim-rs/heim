//! Virtualization system detection.
//!
//! This module is enabled with the `virt` feature flag (enabled by default).
//!
//! At the moment not all declared virtualization systems are detected,
//! therefore this crate should be used very carefully.
//!
//! See the [issues list](https://github.com/heim-rs/heim/issues?q=is%3Aissue+is%3Aopen+label%3AA-virt)
//! for a not supported currently systems.

#![doc(html_root_url = "https://docs.rs/heim-virt/0.0.9")]
#![deny(
    unused,
    unused_imports,
    unused_features,
    bare_trait_objects,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    dead_code,
    deprecated
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]

use heim_common::prelude::*;

mod sys;

/// Virtualization systems (both VMs and containers)
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub enum Virtualization {
    // VMs
    /// Kernel Virtual Machine (https://www.linux-kvm.org)
    Kvm,

    /// QEMU (https://www.qemu.org/)
    Qemu,

    /// Bochs IA-32 emulator (http://bochs.sourceforge.net/)
    Bochs,

    /// Xen project (https://xenproject.org/)
    Xen,

    /// User-Mode Linux (http://user-mode-linux.sourceforge.net/)
    Uml,

    /// VMware (https://www.vmware.com)
    Vmware,

    /// Oracle virtualization (https://www.oracle.com/virtualization/)
    Oracle,

    /// Microsoft Hyper-V (http://www.microsoft.com/hyper-v)
    HyperV,

    /// FreeBSD bhyve (https://wiki.freebsd.org/bhyve)
    Bhyve,
    //    Zvm,
    /// Parallels (https://www.parallels.com/)
    Parallels,

    /// QNX hypervisor (https://blackberry.qnx.com/en/products/hypervisor/index)
    Qnx,

    /// ACRN hypervisor (https://projectacrn.org/)
    Acrn,

    // Containers
    /// `systemd-nspawn` container manager (https://www.freedesktop.org/wiki/Software/systemd/)
    SystemdNspawn,

    /// `lxc-libvirt` (https://libvirt.org/drvlxc.html)
    LxcLibvirt,

    /// Linux Containers (https://linuxcontainers.org/lxc)
    Lxc,

    /// OpenVz (https://openvz.org/)
    OpenVz,

    /// Docker (https://www.docker.com/)
    Docker,

    /// Podman (https://podman.io/)
    Podman,

    /// CoreOS rkt (https://coreos.com/rkt/)
    Rkt,

    /// Microsoft WSL (https://docs.microsoft.com/en-us/windows/wsl/about)
    Wsl,

    /// Unknown virtualization system.
    ///
    /// Usually means that there are symptoms of being running in some virtualization system,
    /// but it can't be determined specifically.
    Unknown,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl Virtualization {
    /// Returns `true` if it is a Virtual Machine virtualization.
    pub fn is_vm(&self) -> bool {
        match self {
            Virtualization::Kvm => true,
            Virtualization::Qemu => true,
            Virtualization::Bochs => true,
            Virtualization::Xen => true,
            Virtualization::Uml => true,
            Virtualization::Vmware => true,
            Virtualization::Oracle => true,
            Virtualization::HyperV => true,
            Virtualization::Bhyve => true,
            Virtualization::Qnx => true,
            Virtualization::Acrn => true,
            _ => false,
        }
    }

    /// Returns `true` if it is a container virtualization.
    pub fn is_container(&self) -> bool {
        match self {
            Virtualization::SystemdNspawn => true,
            Virtualization::LxcLibvirt => true,
            Virtualization::Lxc => true,
            Virtualization::OpenVz => true,
            Virtualization::Docker => true,
            Virtualization::Podman => true,
            Virtualization::Rkt => true,
            Virtualization::Wsl => true,
            _ => false,
        }
    }

    /// Returns string identifying this virtualization system.
    pub fn as_str(&self) -> &str {
        match self {
            Virtualization::Kvm => "kvm",
            Virtualization::Qemu => "qemu",
            Virtualization::Bochs => "bochs",
            Virtualization::Xen => "xen",
            Virtualization::Uml => "uml",
            Virtualization::Vmware => "vmware",
            Virtualization::Oracle => "oracle",
            Virtualization::HyperV => "hyperv",
            Virtualization::Bhyve => "bhyve",
            Virtualization::Qnx => "qnx",
            Virtualization::Acrn => "acrn",
            Virtualization::SystemdNspawn => "systemd-nspawn",
            Virtualization::LxcLibvirt => "lxc-libvirt",
            Virtualization::Lxc => "lxc",
            Virtualization::OpenVz => "openvz",
            Virtualization::Docker => "docker",
            Virtualization::Podman => "podman",
            Virtualization::Rkt => "rkt",
            Virtualization::Wsl => "wsl",
            Virtualization::Unknown => "unknown",
            _ => unreachable!(),
        }
    }
}

/// Returns future which tries to determine if the running program is running
/// in some [Virtualization] system.
///
/// ## Compatibility
///
/// At the moment this function works only for Linux (partially)
/// and always returns `None` for macOS and Windows.
pub fn detect() -> impl Future<Output = Option<Virtualization>> {
    self::sys::detect()
}
