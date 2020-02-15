//! Proc-macros for `heim` crates.
//!
//! Do not use directly.

#![doc(html_root_url = "https://docs.rs/heim-derive/0.0.10")]
#![recursion_limit = "128"]
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

#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;

mod ci;
mod dev;

/// Used for `#[heim_derive::test]`-annotated functions
///
/// Will not run the annotated function if it is called in the CI environment.
///
/// It is important to put it **before** the `#[heim_derive::test]` attribute, like that:
///
/// ```text
/// #[heim_derive::skip_ci]
/// #[heim_derive::test]
/// async fn test_foo() {}
/// ```
///
/// Supported CI:
///  * Azure Pipelines
#[proc_macro_attribute]
pub fn skip_ci(attr: TokenStream, item: TokenStream) -> TokenStream {
    self::ci::skip_ci(attr, item)
}

/// Defines the async main function.
///
/// Same thing what `runtime::main` does, but without checks and with `futures::executor` instead.
///
/// It is used for `heim` examples only.
#[cfg(not(test))]
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    self::dev::main(attr, item)
}

/// Defines the async test function.
///
/// It is used for `heim` test only. See `heim_derive::main` for additional details.
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    self::dev::test(attr, item)
}

/// Defines the async benchmark function.
///
/// It is used for `heim` test only. See `heim_derive::main` for additional details.
#[proc_macro_attribute]
pub fn bench(attr: TokenStream, item: TokenStream) -> TokenStream {
    self::dev::bench(attr, item)
}
