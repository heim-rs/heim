use heim_common::prelude::*;

use std::str::FromStr;

/// Known filesystems.
///
/// All physical filesystems should have their own enum element
/// and all virtual filesystems will go into the `Other` element.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum FileSystem {
    Ext2,
    Ext3,
    Ext4,
    VFat,
    Ntfs,
    Zfs,
    Hfs,
    Reiserfs,
    FuseBlk,

    // TODO: Extend list
    // References:
    //  * https://linux.die.net/man/2/fstatfs
    Other(String),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl FileSystem {
    pub fn is_physical(&self) -> bool {
        match self {
            FileSystem::Other(..) => false,
            _ => true,
        }
    }

    pub fn is_virtual(&self) -> bool {
        !self.is_physical()
    }

    pub fn as_str(&self) -> &str {
        match self {
            FileSystem::Ext2 => "ext2",
            FileSystem::Ext3 => "ext3",
            FileSystem::Ext4 => "ext4",
            FileSystem::VFat => "vfat",
            FileSystem::Ntfs => "ntfs",
            FileSystem::Zfs => "zfs",
            FileSystem::Hfs => "hfs",
            FileSystem::Reiserfs => "reiserfs",
            FileSystem::FuseBlk => "fuseblk",
            FileSystem::Other(string) => string.as_str(),
            _ => unreachable!(),
        }
    }
}

impl FromStr for FileSystem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "ext2" => Ok(FileSystem::Ext2),
            "ext3" => Ok(FileSystem::Ext3),
            "ext4" => Ok(FileSystem::Ext4),
            "vfat" => Ok(FileSystem::VFat),
            "ntfs" => Ok(FileSystem::Ntfs),
            "zfs" => Ok(FileSystem::Zfs),
            "hfs" => Ok(FileSystem::Hfs),
            "reiserfs" => Ok(FileSystem::Reiserfs),
            "fuseblk" => Ok(FileSystem::FuseBlk),
            other => Ok(FileSystem::Other(other.to_string())),
        }
    }
}
