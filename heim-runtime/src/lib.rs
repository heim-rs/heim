//! This crate is a shim around various async runtimes.
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
//!
//! Public API should somewhat match the `std`, `tokio` or `async-std` modules structure.

#![doc(html_root_url = "https://docs.rs/heim-runtime/0.0.6")]
#![allow(
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
#![allow(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]

// This re-export is needed to make `futures::{join, try_join}` macros to work
#[doc(hidden)]
pub use futures;

// Runtimes

cfg_if::cfg_if! {
    if #[cfg(feature = "runtime-tokio")] {
        #[path = "tokio/mod.rs"]
        mod runtime;

        pub use runtime::{join, try_join};
    } else if #[cfg(feature = "runtime-async-std")] {
        #[path = "async_std/mod.rs"]
        mod runtime;
        // `futures` macros are used with `async-std` runtime,
        // because `async-macros` crate macros are not so convenient,
        // as the `tokio` or `futures` ones.
        mod macros;
    } else if #[cfg(feature = "runtime-gio")] {
        // placeholder
        compile_error!("GIO integration is not implemented yet");
    } else {
        #[path = "polyfill/mod.rs"]
        mod runtime;
        mod macros;
    }
}

pub mod fs;
pub mod task;
