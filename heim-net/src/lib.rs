//! Network information.

#![doc(html_root_url = "https://docs.rs/heim-net/0.1.0-rc.1")]
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
    broken_intra_doc_links
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

mod counters;
mod nic;

pub use self::counters::*;
pub use self::nic::*;

// Re-exports
pub use macaddr::{MacAddr, MacAddr6, MacAddr8};
