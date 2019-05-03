//! Crate with futures and streams to determine the memory and swap information
//! available in the current system.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

#![allow(stable_features)]
#![feature(futures_api)]

mod memory;
pub mod os;
mod swap;
mod sys;

pub use self::memory::*;
pub use self::swap::*;

pub use heim_common::units::Information;
