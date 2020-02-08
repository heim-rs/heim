use std::fmt;
use std::path::Path;

use heim_common::prelude::*;
use heim_common::units::{Information, Ratio};

use crate::sys;

/// Disk usage statistics.
///
/// ## Compatibility
///
/// See [os]-specific extension traits also.
///
/// [os]: ./os/index.html
pub struct Usage(sys::Usage);

wrap!(Usage, sys::Usage);

impl Usage {
    /// Returns total information amount available in partition.
    pub fn total(&self) -> Information {
        self.as_ref().total()
    }

    /// Returns used information amount used in partition.
    pub fn used(&self) -> Information {
        self.as_ref().used()
    }

    /// Returns free information about used in partition.
    pub fn free(&self) -> Information {
        self.as_ref().free()
    }

    /// Returns the ratio between used and free information amount in partition.
    pub fn ratio(&self) -> Ratio {
        self.as_ref().ratio()
    }
}

impl fmt::Debug for Usage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Usage")
            .field("total", &self.total())
            .field("used", &self.used())
            .field("free", &self.free())
            .field("ratio", &self.ratio())
            .finish()
    }
}

/// Returns disk [Usage] statistics about the partition which contains the given `path`.
///
/// [Usage]: ./struct.Usage.html
pub fn usage<T>(path: T) -> impl Future<Output = Result<Usage>>
where
    T: AsRef<Path>,
{
    sys::usage(path).map(|res| res.map(Into::into))
}
