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
    broken_intra_doc_links,
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]

use std::future::Future;

pub use futures::pin_mut as pin;

pub mod fs;
pub mod time;

#[inline]
pub async fn spawn<F, R>(f: F) -> R
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    smol::spawn(f).await
}

pub async fn spawn_blocking<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    smol::unblock(f).await
}
