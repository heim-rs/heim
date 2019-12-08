//! System processes information.
//!
//! This module is enabled with the `process` feature flag (enabled by default).

#![doc(html_root_url = "https://docs.rs/heim-process/0.0.9")]
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
#![allow(clippy::missing_safety_doc)]

mod errors;
pub mod os;
mod sys;

mod pids;
mod process;

pub use self::pids::*;
pub use self::process::*;

pub use self::errors::{ProcessError, ProcessResult};
pub use heim_common::units::Time;
pub use heim_common::Pid;

#[cfg(target_os = "linux")]
pub use heim_net::IoCounters;
