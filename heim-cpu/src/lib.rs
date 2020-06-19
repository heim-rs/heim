//! CPU information.
//!
//! ## Platform-specific functions
//!
//! * Average system load fetching is available for *nix systems: [`os::unix::loadavg`]
//!
//! [`os::unix::loadavg`]: ./os/unix/fn.loadavg.html

#![doc(html_root_url = "https://docs.rs/heim-cpu/0.1.0-beta.3")]
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
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod os;
mod sys;

mod count;
mod freq;
mod stats;
mod times;

pub use self::count::*;
pub use self::freq::*;
pub use self::stats::*;
pub use self::times::*;
