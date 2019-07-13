use heim_common::prelude::*;

use std::str::FromStr;

/// Known filesystems.
///
/// All physical filesystems should have their own enum element
/// and all virtual filesystems will go into the `Other` element.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum FileSystem {
    /// ext2 (https://en.wikipedia.org/wiki/Ext2)
    Ext2,

    /// ext3 (https://en.wikipedia.org/wiki/Ext3)
    Ext3,

    /// ext4 (https://en.wikipedia.org/wiki/Ext4)
    Ext4,

    /// FAT (https://en.wikipedia.org/wiki/File_Allocation_Table)
    VFat,

    /// NTFS (https://en.wikipedia.org/wiki/NTFS)
    Ntfs,

    /// ZFS (https://en.wikipedia.org/wiki/ZFS)
    Zfs,

    /// HFS (https://en.wikipedia.org/wiki/Hierarchical_File_System)
    Hfs,

    /// ReiserFS (https://en.wikipedia.org/wiki/ReiserFS)
    Reiserfs,

    // TODO: Should it be considered as a physical FS?
    /// FUSE (https://en.wikipedia.org/wiki/Filesystem_in_Userspace)
    FuseBlk,

    // TODO: Extend list
    // References:
    //  * https://linux.die.net/man/2/fstatfs
    //  * FAT, FAT32, NTFS, HPFS, CDFS, UDF or NWFS
    /// Some unspecified filesystem.
    Other(String),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl FileSystem {
    /// Checks if filesystem is used for a physical devices
    pub fn is_physical(&self) -> bool {
        match self {
            FileSystem::Other(..) => false,
            _ => true,
        }
    }

    /// Checks if filesystem is used for a virtual devices (such as `tmpfs` or `smb` mounts)
    pub fn is_virtual(&self) -> bool {
        !self.is_physical()
    }

    /// Returns a string identifying this filesystem.
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
        match () {
            _ if s.eq_ignore_ascii_case("ext2") => Ok(FileSystem::Ext2),
            _ if s.eq_ignore_ascii_case("ext3") => Ok(FileSystem::Ext3),
            _ if s.eq_ignore_ascii_case("ext4") => Ok(FileSystem::Ext4),
            _ if s.eq_ignore_ascii_case("vfat") => Ok(FileSystem::VFat),
            _ if s.eq_ignore_ascii_case("ntfs") => Ok(FileSystem::Ntfs),
            _ if s.eq_ignore_ascii_case("zfs") => Ok(FileSystem::Zfs),
            _ if s.eq_ignore_ascii_case("hfs") => Ok(FileSystem::Hfs),
            _ if s.eq_ignore_ascii_case("reiserfs") => Ok(FileSystem::Reiserfs),
            _ if s.eq_ignore_ascii_case("fuseblk") => Ok(FileSystem::FuseBlk),
            _ => Ok(FileSystem::Other(s.to_string())),
        }
    }
}
