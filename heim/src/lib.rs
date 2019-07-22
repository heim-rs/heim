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
//! At the moment it is in **MVP** phase, which means that [Tier 1](https://forge.rust-lang.org/platform-support.html#tier-1)
//! platforms only (Linux, macOS and Windows for `i686` and `x86_64`)
//! are **partially** supported.
//! You may want to check out the [GitHub projects page](https://github.com/heim-rs/heim/projects)
//! for more information about cross-platform support.
//!
//! In addition, it would be better to double check if returned information is reliable.
//! You know, just in case.
//!
//! ## Documentation
//!
//! Note that `heim` also is provides platform-specific APIs.
//! If you are browsing this documentation via [docs.rs](https://docs.rs/heim),
//! do not forget to use the platform selector at the page header.

#![doc(html_root_url = "https://docs.rs/heim/0.0.4")]
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
