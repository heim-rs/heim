//! Proc-macros for `heim` crates.
//!
//! Do not use directly.

#![doc(html_root_url = "https://docs.rs/heim-derive/0.0.9")]
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

extern crate proc_macro;

use proc_macro::TokenStream;

mod ci;
mod dev;
mod getters;

/// Augument OS-specific trait with boilerplate-generation.
///
/// Automatically implements this trait for target struct,
/// generates all opaque methods and attaches #\[cfg()\] attribute.
///
/// Should be used as following:
///
/// ```norun
/// #[heim_derive::os_ext_for(crate::CpuTimes, cfg(target_os = "linux"))]
/// pub trait CpuTimesExt {
///     fn foo(&self) -> u32;
/// }
/// ```
///
/// Will generate the code similar to following:
///
/// ```norun
/// pub trait CpuTimesExt {
///     fn foo(&self) -> u32;
/// }
///
/// #[cfg(target_os = "linux")
/// impl CpuTimesExt for crate::CpuTimes {
///     fn foo(&self) -> u32 {
///         self.as_ref().foo()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn os_ext_for(attr: TokenStream, item: TokenStream) -> TokenStream {
    self::dev::os_ext_for(attr, item)
}

/// Augument wrapper around OS-specific implementation struct with conversion traits.
///
/// Will auto-generate `AsRef` and `From` for underline struct with `#\[doc(hidden)\]` attribute
#[proc_macro_derive(ImplWrap)]
pub fn impl_wrap(input: TokenStream) -> TokenStream {
    self::dev::impl_wrap(input)
}

/// Generates getters for all struct fields.
///
/// This is quite similar to `getset` or other crates, but highly specific for `heim` case.
///
/// OS-specific structs are usually very thin and contains `Copy`-able fields, therefore
/// there is no need to return reference to them, it is easier to return a copy (ex. field is u32),
/// that's why generated getters are returning data copies.
///
/// Unfortunately, `getset` crate does not allows this behavior at the moment.
/// If it will, it is better to remove that macro at all.
#[proc_macro_derive(Getter, attributes(getter))]
pub fn impl_getters(input: TokenStream) -> TokenStream {
    self::getters::parse(input)
}

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
