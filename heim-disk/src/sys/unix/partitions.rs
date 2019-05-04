use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::ffi::{CStr, OsStr};
use std::pin::Pin;

use heim_common::prelude::*;

use crate::FileSystem;
use super::bindings;

cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        use crate::os::macos::Flags;
    }
}


#[derive(Debug)]
pub struct Partition {
    device: String,
    fs: FileSystem,
    mount_point: PathBuf,
    flags: libc::uint32_t,
}

impl Partition {
    pub fn device(&self) -> Option<&OsStr> {
        Some(OsStr::new(self.device.as_str()))
    }

    pub fn mount_point(&self) -> &Path {
        self.mount_point.as_path()
    }

    pub fn file_system(&self) -> &FileSystem {
        &self.fs
    }

    pub fn raw_flags(&self) -> libc::uint32_t {
        self.flags
    }
}

// TODO: Since `from` may fail in fact, replace it with a `try_from`
// See `FileSystem::from_str` in the implementation
impl From<libc::statfs> for Partition {
    fn from(stat: libc::statfs) -> Partition {
        let device = unsafe {
            CStr::from_ptr(stat.f_mntfromname.as_ptr()).to_string_lossy().to_string()
        };
        let fs_type = unsafe {
            CStr::from_ptr(stat.f_fstypename.as_ptr()).to_string_lossy()
        };
        let mount_path_raw = unsafe {
            CStr::from_ptr(stat.f_mntonname.as_ptr()).to_string_lossy().to_string()
        };
        let mount_point = PathBuf::from(mount_path_raw);

        let fs = FileSystem::from_str(&fs_type)
            .expect("For some stupid reasons failed to parse FS string");

        Partition {
            device,
            fs,
            mount_point,
            flags: stat.f_flags,
        }
    }
}


pub fn partitions() -> impl Stream<Item = Result<Partition>> {
    future::lazy(|_| {
        match bindings::mounts() {
            Ok(mounts) => {
                let stream = stream::iter(mounts).map(|mount| {
                    Ok(Partition::from(mount))
                });
                Box::pin(stream) as Pin<Box<dyn Stream<Item = _> + Send>>
            },
            Err(e) => {
                Box::pin(stream::once(future::err(e)))
            }
        }
    })
    .flatten_stream()
}

pub fn partitions_physical() -> impl Stream<Item = Result<Partition>> {
    partitions()
        .try_filter_map(|partition| {
            match partition.file_system().is_physical() {
                true => future::ok(Some(partition)),
                false => future::ok(None),
            }
        })
}
