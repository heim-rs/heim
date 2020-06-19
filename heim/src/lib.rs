//! `heim` is a fast and eventually fully-featured async library for the Rust programming language
//! intended to provide any possible information about the system it is running on.
//!
//! At a high level, it provides information about:
//!
//!  * CPU
//!  * Disks
//!  * Host
//!  * Memory
//!  * Networks
//!  * Processes
//!  * Virtualization (*in progress*)
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
//! ## Feature flags
//!
//! Heim uses a set of [feature flags](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section)
//! to reduce the amount of compiled code by selecting only the system components
//! you are planning to use.\
//! All these features are *disabled* by default, see modules list below for available features.
//!
//! Alternatively you can use `full` feature to enable all components at once.
//!
//! ## Documentation
//!
//! Note that `heim` also provides platform-specific APIs.
//! If you are browsing this documentation via [docs.rs](https://docs.rs/heim),
//! do not forget to use the platform selector at the page header.\
//! For a local copy, use `--target` argument to choose your platform.
//!
//! Also, due to Rust [bug #15823](https://github.com/rust-lang/rust/issues/15823),
//! type aliases are not rendered properly across the sub-crates bounds,
//! therefore documentation might look terrible in some places,
//! consider checking the sources or sub-crates documentation in such case.

#![doc(html_root_url = "https://docs.rs/heim/0.1.0-beta.3")]
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
    deprecated,
    intra_doc_link_resolution_failure
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "cpu")]
#[doc(inline)]
#[cfg_attr(docsrs, doc(cfg(feature = "cpu")))]
pub use heim_cpu as cpu;

#[cfg(feature = "disk")]
#[doc(inline)]
#[cfg_attr(docsrs, doc(cfg(feature = "disk")))]
pub use heim_disk as disk;

#[cfg(feature = "host")]
#[doc(inline)]
#[cfg_attr(docsrs, doc(cfg(feature = "host")))]
pub use heim_host as host;

#[cfg(feature = "memory")]
#[doc(inline)]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub use heim_memory as memory;

#[cfg(feature = "net")]
#[doc(inline)]
#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
pub use heim_net as net;

#[cfg(feature = "process")]
#[doc(inline)]
#[cfg_attr(docsrs, doc(cfg(feature = "process")))]
pub use heim_process as process;

#[cfg(feature = "virt")]
#[doc(inline)]
#[cfg_attr(docsrs, doc(cfg(feature = "virt")))]
pub use heim_virt as virt;

#[cfg(feature = "sensors")]
#[doc(inline)]
#[cfg_attr(docsrs, doc(cfg(feature = "sensors")))]
pub use heim_sensors as sensors;

pub use heim_common::units;
pub use heim_common::{Error, Result};
