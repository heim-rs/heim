//! Sensors information.

#![doc(html_root_url = "https://docs.rs/heim-sensors/0.1.0-beta.1")]
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
#![cfg_attr(docsrs, feature(doc_cfg))]

mod sys;

mod temperatures;

pub use self::temperatures::*;
