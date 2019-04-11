//! Optional tokio runtime for projects that want to use `heim` synchronously.
//!
//! If you have your own `tokio` runtime (directly, or via `warp`, `actix` or whatever),
//! you do not need to use this crate.

use std::io;

mod runtime;

pub use self::runtime::*;

/// Creates some "default" runtime which can be used for synchronously working programs.
///
/// At the moment it is very experimental and is based on the `Tokio` PR
/// [#1045](https://github.com/tokio-rs/tokio/pull/1045).
/// One additional system thread will be created and used for blocking IO operations.
pub fn new() -> io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new()
        .core_threads(1)
        .blocking_threads(1)
        .name_prefix("heim-runtime-")
        .build()
}
