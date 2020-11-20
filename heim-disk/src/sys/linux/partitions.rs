use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use heim_common::prelude::*;
use heim_runtime as rt;

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
    type Err = Error;

    fn from_str(line: &str) -> Result<Partition> {
        // Example: `/dev/sda3 /home ext4 rw,relatime,data=ordered 0 0`
        let mut parts = line.splitn(5, ' ');
        let mount_root = rt::linux::procfs_root().join("mounts");
        let device = match parts.next() {
            Some(device) if device == "none" => None,
            Some(device) => Some(device.to_string()),
            None => {
                return Err(Error::missing_key(
                    "device",
                    format!("{}", mount_root.display()),
                ))
            }
        };
        let mount_point = match parts.next() {
            Some(point) => PathBuf::from(point),
            None => {
                return Err(Error::missing_key(
                    "mount point",
                    format!("{}", mount_root.display()),
                ))
            }
        };
        let fs_type = match parts.next() {
            Some(fs) => FileSystem::from_str(fs)?,
            _ => {
                return Err(Error::missing_key(
                    "file-system type",
                    format!("{}", mount_root.display()),
                ))
            }
        };
        let options = match parts.next() {
            Some(opts) => opts.to_string(),
            None => {
                return Err(Error::missing_key(
                    "options",
                    format!("{}", mount_root.display()),
                ))
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
async fn known_filesystems() -> Result<HashSet<FileSystem>> {
    rt::spawn_blocking(|| {
        let file = fs::File::open(rt::linux::procfs_root().join("filesystems"))?;
        let reader = io::BufReader::new(file);
        let mut acc = HashSet::with_capacity(4);

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.splitn(2, '\t');

            #[allow(clippy::match_like_matches_macro)] // >= 1.42.0
            let nodev = match parts.next() {
                Some("nodev") => true,
                _ => false,
            };

            let fs = match parts.next() {
                Some("zfs") if nodev => FileSystem::from_str("zfs"),
                Some(filesystem) if !nodev => FileSystem::from_str(filesystem),
                _ => continue,
            }?;

            let _ = acc.insert(fs);
        }

        Ok(acc)
    })
    .await
}

pub async fn partitions() -> Result<impl Stream<Item = Result<Partition>>> {
    let lines = rt::fs::read_lines(rt::linux::procfs_root().join("mounts")).await?;
    let stream = lines
        .map_err(Error::from)
        .try_filter_map(|line| async move {
            let result = Partition::from_str(&line).ok();

            Ok(result)
        });

    Ok(stream)
}

pub async fn partitions_physical() -> Result<impl Stream<Item = Result<Partition>>> {
    let filesystems = known_filesystems().await?;
    let stream = partitions().await?;

    let stream = stream.try_filter_map(move |part| match part {
        Partition { device: None, .. } => future::ok(None),
        Partition { ref fs_type, .. } if !filesystems.contains(fs_type) => future::ok(None),
        partition => future::ok(Some(partition)),
    });

    Ok(stream)
}
