//! Network information.

#![doc(html_root_url = "https://docs.rs/heim-net/0.1.0-beta.1")]
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
#![allow(
    // Next two are temporary, see https://github.com/rust-lang/rust/issues/72686
    unknown_lints,
    unused_crate_dependencies,

    clippy::missing_safety_doc
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod os;
mod sys;

mod counters;
mod nic;

pub use self::counters::*;
pub use self::nic::*;

// Re-exports
pub use macaddr::{MacAddr, MacAddr6, MacAddr8};
