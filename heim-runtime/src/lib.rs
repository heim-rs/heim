#![doc(html_root_url = "https://docs.rs/heim-runtime/0.1.0-beta.1")]
#![deny(
    unused,
    unused_imports,
    unused_features,
    unsafe_code,
    bare_trait_objects,
    future_incompatible,
    missing_debug_implementations,
//    missing_docs, // TODO
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
    unused_crate_dependencies
)]
use std::future::Future;

pub mod fs;
pub mod time;

#[inline]
pub async fn spawn<F, R>(f: F) -> R
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    smol::Task::spawn(f).await
}

pub async fn spawn_blocking<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    smol::Task::blocking(async move { f() }).await
}
