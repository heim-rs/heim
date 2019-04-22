use std::fmt;
use std::path::Path;

use heim_common::prelude::*;

use crate::{sys, FileSystem};

/// Mounted disk partition.
#[derive(heim_derive::ImplWrap)]
pub struct Partition(sys::Partition);

impl Partition {
    // TODO: Should return `Option<OsStr>`, since device name might vary differently
    pub fn device(&self) -> Option<&str> {
        self.as_ref().device()
    }

    pub fn mount_point(&self) -> &Path {
        self.as_ref().mount_point()
    }

    /// Returns partition file system.
    pub fn file_system(&self) -> &FileSystem {
        self.as_ref().file_system()
    }

    /// Returns mount options.
    ///
    /// Since options are widely different from system to system,
    /// at the moment they are returned as a `&str`.
    pub fn options(&self) -> &str {
        self.as_ref().options()
    }
}

impl fmt::Debug for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Partition")
            .field("device", &self.device())
            .field("mount_point", &self.mount_point())
            .field("file_system", &self.file_system())
            .field("options", &self.options())
            .finish()
    }
}

/// Returns stream which yields mounted disk [Partitions].
///
/// This includes all virtual partitions, such as `tmpfs`.
/// See [partitions_physical] for physical partitions stream.
///
/// [Partitions]: struct.Partition.html
pub fn partitions() -> impl Stream<Item = Partition, Error = Error> {
    sys::partitions().map(Into::into)
}

/// Returns stream which yields physical only mounted disk [Partitions].
///
/// [Partitions]: struct.Partition.html
pub fn partitions_physical() -> impl Stream<Item = Partition, Error = Error> {
    sys::partitions_physical().map(Into::into)
}
