//! System processes information.

#![doc(html_root_url = "https://docs.rs/heim-process/0.1.1-beta.1")]
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
    unused_crate_dependencies,

    clippy::missing_safety_doc
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

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
