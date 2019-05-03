//! Somewhat-safe wrappers around the IOKit stuff.
//!
//! Intended to be used only across the `heim` project.
//! There is no guarantees on stability or whatever, if you are going to use it directly
//! in other project. Seriously, do not do it.

mod ffi;
mod io_iterator;
mod io_master_port;
mod io_object;
mod properties;

pub use self::io_iterator::IoIterator;
pub use self::io_master_port::IoMasterPort;
pub use self::io_object::IoObject;
pub use self::properties::DictionaryProps;
