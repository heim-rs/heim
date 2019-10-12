use std::io;
use std::path::Path;

#[cfg(target_os = "windows")]
use std::os::windows::io::RawHandle;

use futures_util::TryFutureExt;

use crate::shims::fs as shims;

/// A reference to an open file in filesystem.
#[derive(Debug)]
pub struct File(shims::File);

impl File {
    /// Attempt to open file in read-only mode.
    pub async fn open<T>(path: T) -> io::Result<File>
    where
        T: AsRef<Path>,
    {
        shims::File::open(path).map_ok(File).await
    }

    /// Returns the raw Windows handle from file.
    #[cfg(target_os = "windows")]
    pub fn as_raw_handle(&self) -> RawHandle {
        self.0.as_raw_handle()
    }
}
