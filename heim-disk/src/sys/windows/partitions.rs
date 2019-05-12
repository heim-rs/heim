use std::pin::Pin;
use std::path::{Path, PathBuf};
use std::ffi::{OsString, OsStr};

use heim_common::prelude::*;

use crate::FileSystem;
use crate::os::windows::{Flags, DriveType};
use super::bindings::disks2;

pub struct Partition {
    // Might be missing for a remote FS, such as SMB
    volume: Option<OsString>,
    mount_point: PathBuf,
    drive_type: Option<DriveType>,
    flags: Flags,
    file_system: FileSystem,
}

impl Partition {
    pub fn device(&self) -> Option<&OsStr> {
        self.volume.as_ref().map(OsString::as_os_str)
    }

    pub fn mount_point(&self) -> &Path {
        self.mount_point.as_path()
    }

    pub fn file_system(&self) -> &FileSystem {
        &self.file_system
    }

    pub fn flags(&self) -> Flags {
        self.flags
    }
}

pub fn partitions() -> impl Stream<Item = Result<Partition>> {
    future::lazy(|_| {
        let disks = disks2::LogicalDrives::new()?;

        let stream = stream::iter(disks).map(Ok);
        Ok(Box::pin(stream) as Pin<Box<dyn Stream<Item = _> + Send>>)
    })
    .unwrap_or_else(|e: Error| Box::pin(stream::once(future::err(e))))
    .flatten_stream()
    .and_then(|disk: disks2::LogicalDrive| {
        match disk.information() {
            Ok(Some((flags, fs))) => {
                future::ok(Some(Partition {
                    volume: disk.volume_name().ok(),
                    mount_point: PathBuf::from(disk.to_os_string()),
                    drive_type: disk.drive_type(),
                    file_system: fs,
                    flags,
                }))
            },
            Ok(None) => future::ok(None),
            Err(e) => future::err(e)
        }
    })
    .try_filter_map(future::ok)
}

pub fn partitions_physical() -> impl Stream<Item = Result<Partition>> {
    partitions()
        .try_filter(|drive| {
            let result = match drive.drive_type {
                Some(DriveType::NoRootDir) => false,
                Some(DriveType::Remote) => false,
                Some(DriveType::RamDisk) => false,
                None => false,
                _ => true,
            };

            future::ready(result)
        })
}
