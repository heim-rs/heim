//! `heim` is a fast and eventually fully-featured async library for the Rust programming language
//! intended to provide any possible information about the system it is running on.
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
//!  * Hardware sensors (*in progress*)
//!
//! ## Platform support
//!
//! At the moment, `heim` is in **MVP** phase, which means that there is only only **partial** support
//! for [Tier 1](https://forge.rust-lang.org/platform-support.html#tier-1)
//! platforms (Linux, macOS, and Windows for `i686` and `x86_64`).
//! You can check the [GitHub projects page](https://github.com/heim-rs/heim/projects)
//! for more information.
//!
//! In addition, it would be good to double check if the returned information is correct.
//! You know, just in case.
//!
//! ## Documentation
//!
//! Note that `heim` also provides platform-specific APIs.
//! If you are browsing this documentation via [docs.rs](https://docs.rs/heim),
//! do not forget to use the platform selector at the page header.
//!
//! Also, due to Rust [bug #15823](https://github.com/rust-lang/rust/issues/15823),
//! type aliases are not rendered properly across the sub-crates bounds,
//! therefore documentation might look terrible in some places,
//! consider checking the sources or sub-crates documentation in such case.

#![doc(html_root_url = "https://docs.rs/heim/0.0.9")]
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

#[cfg(feature = "sensors")]
#[doc(inline)]
pub use heim_sensors as sensors;

pub use heim_common::units;
pub use heim_common::{Error, Result};
