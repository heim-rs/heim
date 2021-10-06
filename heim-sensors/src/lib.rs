//! Sensors information.

#![doc(html_root_url = "https://docs.rs/heim-sensors/0.1.0-rc.1")]
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
    rustdoc::broken_intra_doc_links
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
