use std::ffi::OsString;
use std::path::PathBuf;

use crate::shims::fs;

/// Dir entry
#[derive(Debug)]
pub struct DirEntry(fs::DirEntry);

impl DirEntry {
    /// Returns path to that dir entry
    pub fn path(&self) -> PathBuf {
        self.0.path()
    }

    /// Returns dir entry file name
    pub fn file_name(&self) -> OsString {
        self.0.file_name()
    }
}

impl From<fs::DirEntry> for DirEntry {
    fn from(entry: fs::DirEntry) -> Self {
        Self(entry)
    }
}
