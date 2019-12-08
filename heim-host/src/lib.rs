//! Host system information.
//!
//! This module is enabled with the `host` feature flag (enabled by default).

#![doc(html_root_url = "https://docs.rs/heim-host/0.0.9")]
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

#[cfg(target_os = "windows")]
#[macro_use]
extern crate winapi;

pub use platforms::target::Arch;

mod sys;

mod boot_time;
pub mod os;
mod platform;
mod uptime;
mod users;

pub use self::boot_time::*;
pub use self::platform::*;
pub use self::uptime::*;
pub use self::users::*;

pub use heim_common::units::Time;
pub use heim_common::Pid;
