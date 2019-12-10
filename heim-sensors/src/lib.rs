//! Sensors information.
//!
//! This module is enabled with the `sensors` feature flag (enabled by default).

#![doc(html_root_url = "https://docs.rs/heim-sensors/0.0.4")]
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

mod sys;

mod temperatures;

pub use self::temperatures::*;
