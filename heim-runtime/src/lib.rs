//! This crate is a shim around various async runtimes with a fallback to sync operations.
//!
//! ## Why?
//!
//! It might be expected that an async library should be a runtime-agnostic thing,
//! but due to current ecosystem state it is not possible yet;
//! for example there is no async files I/O abstraction.
//!
//! Until then, this crate will provide the wrappers around the various reactors, if possible,
//! and end users may choose the implementation, which is compatible with their reactor.
//!
//! See also: https://github.com/heim-rs/heim/issues/75

#![doc(html_root_url = "https://docs.rs/heim-runtime/0.0.5")]
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

mod shims;

pub mod fs;
