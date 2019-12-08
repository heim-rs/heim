//! Disks information.
//!
//! This module is enabled with the `disk` feature flag (enabled by default).

#![doc(html_root_url = "https://docs.rs/heim-disk/0.0.9")]
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
#![allow(clippy::missing_safety_doc)]

pub mod os;
mod sys;

mod counters;
mod filesystem;
mod partitions;
mod usage;

pub use self::counters::*;
pub use self::filesystem::*;
pub use self::partitions::*;
pub use self::usage::*;

pub use heim_common::units::Information;
pub use heim_common::units::Ratio;
