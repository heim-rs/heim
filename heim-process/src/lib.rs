//! System processes information.
//!
//! This module is enabled with the `process` feature flag (enabled by default).

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

mod errors;
mod sys;

mod pids;

pub use self::pids::*;

pub use self::errors::ProcessError;

/// Process identifier type.
#[cfg(not(target_os = "windows"))]
pub type Pid = libc::pid_t;

/// Process identifier type.
// TODO: Is it a correct type for pid?
#[cfg(target_os = "windows")]
pub type Pid = winapi::shared::minwindef::DWORD;
