//! `Heim` is a fast and eventually full-featured framework for Rust programming language
//! intended to provide any possible information about the system it is running.
//!
//! `heim` is designed to be async-first, but also can be used in the synchronous contexts
//! with a little drawbacks (see [runtime] module for details).
//!
//! [runtime]: ./runtime/index.html
//!
//! ## Compatibility
//!
//! At the moment `heim` is in **MVP** phase, which means that only Linux is supported.
//! You may want to check out [GitHub projects](https://github.com/heim-rs/heim/projects)
//! for more information about cross-platform support.
//!
//! ## Documentation issue
//!
//! Due to the Rust bug [#15823](https://github.com/rust-lang/rust/issues/15823)
//! re-exported type aliases are not documented properly, so function signatures
//! might be messed up a little, when reading documentation for `heim` crate.
//! Consider using documentation for sub-crates until this bug will be fixed.

#![deny(unused)]
#![warn(missing_docs)]
#![deny(bare_trait_objects)]

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

///// Network information
//pub mod net {
//    pub use heim_net::*;
//}

pub use heim_common::{Error, ErrorKind, Result};
