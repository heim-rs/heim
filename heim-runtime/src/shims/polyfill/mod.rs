//! Polyfill implementation for async operations.
//!
//! Not as fast as other possible options, but at least will not block the current thread at all.
//! Internally uses thread pool to drive futures to completion.
//!
//! This "runtime" will be replaced with the `async-std` version
//! as soon as `async-std` crate will be usable on stable Rust.

pub mod fs;
mod pool;
