//! Memory and swap information.
//!
//! This module is enabled with the `memory` feature flag (enabled by default).

#![doc(html_root_url = "https://docs.rs/heim-memory/0.0.10")]
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

mod memory;
pub mod os;
mod swap;
mod sys;

pub use self::memory::*;
pub use self::swap::*;
