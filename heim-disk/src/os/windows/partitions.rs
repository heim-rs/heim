use winapi::shared::minwindef::DWORD;
use winapi::um::winnt;

/// Windows-specific drive type.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum DriveType {
    /// CD-ROM drive
    CdRom,
    /// Drive is fixed media; for example, a hard disk drive or a flash drive
    Fixed,
    /// The root path is invalid; for example, there is no volume mounted at the specified path.
    NoRootDir,
    /// RAM disk
    RamDisk,
    /// Drive is a remote (network) disk
    Remote,
    /// Drive has removable media; for example, a floppy drive, thumb drive, or flash card reader.
    Removable,
}

bitflags::bitflags! {
    /// Windows volume file system flags.
    ///
    /// Reference: https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getvolumeinformationw
    pub struct Flags: DWORD {
        /// The specified volume supports preserved case of file names when it places a name on disk.
        const FILE_CASE_PRESERVED_NAMES = winnt::FILE_CASE_PRESERVED_NAMES;

        /// The specified volume supports case-sensitive file names.
        const FILE_CASE_SENSITIVE_SEARCH = winnt::FILE_CASE_SENSITIVE_SEARCH;

        /// The specified volume is a direct access (DAX) volume.
        const FILE_DAX_VOLUME = winnt::FILE_DAX_VOLUME;

        /// The specified volume supports file-based compression.
        const FILE_FILE_COMPRESSION = winnt::FILE_FILE_COMPRESSION;

        /// The specified volume supports named streams.
        const FILE_NAMED_STREAMS = winnt::FILE_NAMED_STREAMS;

        /// The specified volume preserves and enforces access control lists (ACL).
        const FILE_PERSISTENT_ACLS = winnt::FILE_PERSISTENT_ACLS;

        /// The specified volume is read-only.
        const FILE_READ_ONLY_VOLUME = winnt::FILE_READ_ONLY_VOLUME;

        /// The specified volume supports a single sequential write.
        const FILE_SEQUENTIAL_WRITE_ONCE = winnt::FILE_SEQUENTIAL_WRITE_ONCE;

        /// The specified volume supports the Encrypted File System (EFS).
        const FILE_SUPPORTS_ENCRYPTION = winnt::FILE_SUPPORTS_ENCRYPTION;

        /// The specified volume supports extended attributes.
        const FILE_SUPPORTS_EXTENDED_ATTRIBUTES = winnt::FILE_SUPPORTS_EXTENDED_ATTRIBUTES;

        /// The specified volume supports hard links. For more information, see Hard Links and Junctions.
        const FILE_SUPPORTS_HARD_LINKS = winnt::FILE_SUPPORTS_HARD_LINKS;

        /// The specified volume supports object identifiers.
        const FILE_SUPPORTS_OBJECT_IDS = winnt::FILE_SUPPORTS_OBJECT_IDS;

        /// The file system supports open by FileID.
        const FILE_SUPPORTS_OPEN_BY_FILE_ID = winnt::FILE_SUPPORTS_OPEN_BY_FILE_ID;

        /// The specified volume supports reparse points.
        const FILE_SUPPORTS_REPARSE_POINTS = winnt::FILE_SUPPORTS_REPARSE_POINTS;

        /// The specified volume supports sparse files.
        const FILE_SUPPORTS_SPARSE_FILES = winnt::FILE_SUPPORTS_SPARSE_FILES;

        /// The specified volume supports transactions.
        const FILE_SUPPORTS_TRANSACTIONS = winnt::FILE_SUPPORTS_TRANSACTIONS;

        /// The specified volume supports update sequence number (USN) journals.
        const FILE_SUPPORTS_USN_JOURNAL = winnt::FILE_SUPPORTS_USN_JOURNAL;

        /// The specified volume supports Unicode in file names as they appear on disk.
        const FILE_UNICODE_ON_DISK = winnt::FILE_UNICODE_ON_DISK;

        /// The specified volume is a compressed volume, for example, a DoubleSpace volume.
        const FILE_VOLUME_IS_COMPRESSED = winnt::FILE_VOLUME_IS_COMPRESSED;

        /// The specified volume supports disk quotas.
        const FILE_VOLUME_QUOTAS = winnt::FILE_VOLUME_QUOTAS;
    }
}

/// Extension for [Partition] struct.
///
/// [Partition]: ../../struct.Partition.html
pub trait PartitionExt {
    /// Gets mount flags for this partition.
    fn flags(&self) -> Flags;

    /// Get drive type for this partition, if can be determined.
    fn drive_type(&self) -> Option<DriveType>;
}

#[cfg(target_os = "windows")]
impl PartitionExt for crate::Partition {
    fn flags(&self) -> Flags {
        self.as_ref().flags()
    }

    fn drive_type(&self) -> Option<DriveType> {
        self.as_ref().drive_type()
    }
}
