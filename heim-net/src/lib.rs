//! Network information.
//!
//! This module is enabled with the `net` feature flag (enabled by default).

#![doc(html_root_url = "https://docs.rs/heim-net/0.0.9")]
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

pub mod os;
mod sys;

mod counters;
mod nic;

pub use self::counters::*;
pub use self::nic::*;

// Re-exports
pub use macaddr::{MacAddr, MacAddr6, MacAddr8};
