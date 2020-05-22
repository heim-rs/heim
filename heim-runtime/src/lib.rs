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
//!
//! ## Big and important note
//!
//! This crate is used internally by `heim`.
//! No one else should use it, all things in here should be considered as private,
//! and there are no guarantees provided on API stability.
//! Seriously, do not use it. Even if you want to. Don't.

#![doc(html_root_url = "https://docs.rs/heim-runtime/0.1.0-alpha.1")]
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
    deprecated,
    intra_doc_link_resolution_failure
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
pub use futures_util;

// Runtimes

#[cfg(any(
    all(feature = "runtime-polyfill", feature = "runtime-tokio"),
    all(feature = "runtime-polyfill", feature = "runtime-async-std"),
    all(feature = "runtime-tokio", feature = "runtime-async-std"),
))]
compile_error!("Multiple async runtime features are enabled!");

cfg_if::cfg_if! {
    if #[cfg(feature = "runtime-tokio")] {
        #[path = "tokio/mod.rs"]
        mod runtime;

        pub use runtime::{join, try_join, pin};
    } else if #[cfg(feature = "runtime-async-std")] {
        #[path = "async_std/mod.rs"]
        mod runtime;
        // `futures` macros are used with `async-std` runtime,
        // because `async-macros` crate macros are not so convenient,
        // as the `tokio` or `futures` ones.
        pub use pin_utils::pin_mut as pin;
        pub use futures_util::{try_join, join};
    } else if #[cfg(feature = "runtime-polyfill")] {
        #[path = "polyfill/mod.rs"]
        mod runtime;
        pub use pin_utils::pin_mut as pin;
        pub use futures_util::{try_join, join};
    } else {
        compile_error!("None of the async runtime support features were enabled!");
    }
}

pub mod fs;
pub mod task;
pub mod time;
