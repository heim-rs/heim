use std::collections::HashSet;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use heim_common::prelude::*;
use heim_runtime::fs;

use crate::FileSystem;

#[derive(Debug)]
pub struct Partition {
    device: Option<String>,
    mount_point: PathBuf,
    fs_type: FileSystem,
    options: String,
}

impl Partition {
    pub fn device(&self) -> Option<&OsStr> {
        self.device
            .as_ref()
            .map(|device| OsStr::new(device.as_str()))
    }

    pub fn mount_point(&self) -> &Path {
        self.mount_point.as_path()
    }

    pub fn file_system(&self) -> &FileSystem {
        &self.fs_type
    }

    pub fn options(&self) -> &str {
        &self.options
    }
}

impl FromStr for Partition {
    type Err = Error2;

    fn from_str(line: &str) -> Result2<Partition> {
        // Example: `/dev/sda3 /home ext4 rw,relatime,data=ordered 0 0`
        let mut parts = line.splitn(5, ' ');
        let device = match parts.next() {
            Some(device) if device == "none" => None,
            Some(device) => Some(device.to_string()),
            None => {
                let inner = io::Error::from(io::ErrorKind::InvalidData);
                return Err(Error2::from(inner).with_message("Missing device"));
            }
        };
        let mount_point = match parts.next() {
            Some(point) => PathBuf::from(point),
            None => {
                let inner = io::Error::from(io::ErrorKind::InvalidData);
                return Err(Error2::from(inner).with_message("Missing mount point"));
            }
        };
        let fs_type = match parts.next() {
            Some(fs) => FileSystem::from_str(fs)?,
            None => {
                let inner = io::Error::from(io::ErrorKind::InvalidData);
                return Err(Error2::from(inner).with_message("Missing fs type"));
            }
        };
        let options = match parts.next() {
            Some(opts) => opts.to_string(),
            None => {
                let inner = io::Error::from(io::ErrorKind::InvalidData);
                return Err(Error2::from(inner).with_message("Missing mount options"));
            }
        };

        Ok(Partition {
            device,
            mount_point,
            fs_type,
            options,
        })
    }
}

// Returns stream with known physical (only!) partitions
async fn known_filesystems() -> Result2<HashSet<FileSystem>> {
    let mut acc = HashSet::with_capacity(10);

    let mut lines = fs::read_lines("/proc/filesystems").await?;
    while let Some(line) = lines.next().await {
        let line = line?;
        let mut parts = line.splitn(2, '\t');
        let nodev = match parts.next() {
            Some("nodev") => true,
            _ => false,
        };

        let result = match parts.next() {
            Some("zfs") if nodev => FileSystem::from_str("zfs"),
            Some(filesystem) if !nodev => FileSystem::from_str(filesystem),
            _ => continue,
        };

        if let Ok(fs) = result {
            let _ = acc.insert(fs);
        }
    }

    Ok(acc)
}

pub fn partitions() -> impl Stream<Item = Result2<Partition>> {
    fs::read_lines("/proc/mounts")
        .try_flatten_stream()
        .map_err(Error2::from)
        .try_filter_map(|line| {
            let result = Partition::from_str(&line).ok();

            future::ok(result)
        })
}

pub fn partitions_physical() -> impl Stream<Item = Result2<Partition>> {
    known_filesystems()
        .map_ok(|fs| {
            partitions().try_filter_map(move |part| match part {
                Partition { device: None, .. } => future::ok(None),
                Partition { ref fs_type, .. } if !fs.contains(fs_type) => future::ok(None),
                partition => future::ok(Some(partition)),
            })
        })
        .try_flatten_stream()
}
