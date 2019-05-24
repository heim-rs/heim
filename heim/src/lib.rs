//! `Heim` is a fast and eventually full-featured framework for Rust programming language
//! intended to provide any possible information about the system it is running.
//!
//! ## Compatibility
//!
//! At the moment `heim` is in **MVP** phase, which means that only the big triple
//! (Linux, MacOS and Windows) are supported.
//! You may want to check out [GitHub projects](https://github.com/heim-rs/heim/projects)
//! for more information about cross-platform support.

#![allow(stable_features)] // Used for `future_api` feature
#![forbid(unused)]
#![forbid(bare_trait_objects)]
#![forbid(missing_docs)]

/// CPU information
pub mod cpu {
    pub use heim_cpu::*;
}

/// Disk information
pub mod disk {
    pub use heim_disk::*;
}

/// Host information
pub mod host {
    pub use heim_host::*;
}

/// Memory information
pub mod memory {
    pub use heim_memory::*;
}

/// Network information
pub mod net {
    pub use heim_net::*;
}

pub use heim_common::{Error, ErrorKind, Result};
