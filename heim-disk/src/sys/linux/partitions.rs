use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use heim_common::prelude::*;

use crate::FileSystem;

#[derive(Debug)]
pub struct Partition {
    device: Option<String>,
    mount_point: PathBuf,
    fs_type: FileSystem,
    options: String,
}

impl Partition {
    pub fn device(&self) -> Option<&str> {
        self.device.as_ref().map(String::as_str)
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
        let device = match parts.next() {
            Some(device) if device == "none" => None,
            Some(device) => Some(device.to_string()),
            None => return Err(Error::new(ErrorKind::Parse)),
        };
        let mount_point = match parts.next() {
            Some(point) => PathBuf::from(point),
            None => return Err(Error::new(ErrorKind::Parse)),
        };
        let fs_type = match parts.next() {
            Some(fs) => match FileSystem::from_str(fs) {
                Ok(fs) => fs,
                Err(e) => return Err(e),
            },
            _ => return Err(Error::new(ErrorKind::Parse)),
        };
        let options = match parts.next() {
            Some(opts) => opts.to_string(),
            None => return Err(Error::new(ErrorKind::Parse)),
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
fn known_filesystems() -> impl Stream<Item = FileSystem, Error = Error> {
    utils::fs::read_lines("/proc/filesystems").filter_map(|line| {
        let mut parts = line.splitn(2, '\t');
        let nodev = match parts.next() {
            Some("nodev") => true,
            _ => false,
        };

        let fs = match parts.next() {
            Some("zfs") if nodev => FileSystem::from_str("zfs"),
            Some(filesystem) if !nodev => FileSystem::from_str(filesystem),
            _ => return None,
        };

        fs.ok()
    })
}

pub fn partitions() -> impl Stream<Item = Partition, Error = Error> {
    utils::fs::read_lines("/proc/mounts").filter_map(|line| match Partition::from_str(&line) {
        Ok(partition) => Some(partition),
        Err(e) => {
            println!("{:?}", e);
            None
        }, // TODO: trace!
    })
}

pub fn partitions_physical() -> impl Stream<Item = Partition, Error = Error> {
    known_filesystems()
        .collect()
        .map(HashSet::from_iter)
        .map(|fs: HashSet<FileSystem>| {
            partitions().filter(move |part| match part {
                Partition {
                    device: None, ..
                } => false,
                Partition {
                    ref fs_type, ..
                } if !fs.contains(fs_type) => false,
                _ => true,
            })
        })
        .flatten_stream()
}
