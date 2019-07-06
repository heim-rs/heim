//! `heim` is fast and eventually full-featured async framework for Rust programming language
//! intended to provide any possible information about the system it is running.
//!
//! At a high level, it provides information about:
//!
//!  * CPU
//!  * Disks
//!  * Host
//!  * Memory
//!  * Networks (*in progress*)
//!  * Processes (*in progress*)
//!  * Virtualization (*in progress*)
//!  * Windows services (*in progress*)
//!
//! ## Platform support
//!
//! At the moment `heim` is in **MVP** phase, which means that only the big triple
//! (Linux, MacOS and Windows) are **partially** supported.
//! You may want to check out [GitHub projects page](https://github.com/heim-rs/heim/projects)
//! for more information about cross-platform support.
//!
//! In addition, it would be better to double check if returned information is reliable.
//! You know, just in case.

#![forbid(
    unused,
    unstable_features,
    bare_trait_objects,
    future_incompatible,
    missing_debug_implementations,
    nonstandard_style
)]

#[cfg(feature = "cpu")]
#[doc(inline)]
pub use heim_cpu as cpu;

#[cfg(feature = "disk")]
#[doc(inline)]
pub use heim_disk as disk;

#[cfg(feature = "host")]
#[doc(inline)]
pub use heim_host as host;

#[cfg(feature = "memory")]
#[doc(inline)]
pub use heim_memory as memory;

#[cfg(feature = "net")]
#[doc(inline)]
pub use heim_net as net;

#[cfg(feature = "process")]
#[doc(inline)]
pub use heim_process as process;

#[cfg(feature = "virt")]
#[doc(inline)]
pub use heim_virt as virt;

pub use heim_common::{Error, Result};
