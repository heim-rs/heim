//! `heim` is fast and eventually full-featured async framework for Rust programming language
//! intended to provide any possible information about the system it is running.
//!
//! At a high level, it provides information about:
//!
//!  * CPU
//!  * Disks
//!  * Host
//!  * Memory
//!  * Networks (*TBD*)
//!  * Processes (*TBD*)
//!  * Virtualization
//!  * Windows services (*TBD*)
//!
//! ## Platform support
//!
//! At the moment `heim` is in **MVP** phase, which means that only the big triple
//! (Linux, MacOS and Windows) are **partially** supported.
//! You may want to check out [GitHub projects](https://github.com/heim-rs/heim/projects)
//! for more information about cross-platform support.
//!
//! In addition, it would be better to double check if returned information is reliable.
//! You know, just in case.

#![allow(stable_features)] // Used for `future_api` feature
#![forbid(unused)]
#![forbid(bare_trait_objects)]
#![forbid(missing_docs)]

/// CPU information
#[cfg(feature = "cpu")]
pub mod cpu {
    pub use heim_cpu::*;
}

/// Disk information
#[cfg(feature = "disk")]
pub mod disk {
    pub use heim_disk::*;
}

/// Host information
#[cfg(feature = "host")]
pub mod host {
    pub use heim_host::*;
}

/// Memory information
#[cfg(feature = "memory")]
pub mod memory {
    pub use heim_memory::*;
}

/// Virtualization system information
#[cfg(feature = "virt")]
pub mod virt {
    pub use heim_virt::*;
}

pub use heim_common::{Error, Result};
