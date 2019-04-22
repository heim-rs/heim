bitflags::bitflags! {
    // These flags are declared at `bsd/sys/mount.h`
    pub struct Flags: libc::c_uint {
        // User specifiable flags

        /// Read only filesystem
        const MNT_RDONLY = 0x0000_0001;
        /// File system written synchronously
        const MNT_SYNCHRONOUS = 0x0000_0002;
        /// Can't exec from filesystem
        const MNT_NOEXEC = 0x0000_0004;
        /// Don't honor `setuid` bits on fs
        const MNT_NOSUID = 0x0000_0008;
        /// Don't interpret special files
        const MNT_NODEV = 0x0000_0010;
        /// Union with underlying filesystem
        const MNT_UNION = 0x0000_0020;
        /// File system written asynchronously
        const MNT_ASYNC = 0x0000_0040;
        /// File system supports content protection
        const MNT_CPROTECT = 0x0000_0080;

        // NFS export related mount flags

        /// File system is exported
        const MNT_EXPORTED = 0x0000_0100;

        // MAC labeled / "quarantined" flag

        /// File system is quarantined
        const MNT_QUARANTINE = 0x0000_0400;

        // Flags set by internal operations

        /// Filesystem is stored locally
        const MNT_LOCAL = 0x0000_1000;
        /// Quotas are enabled on filesystem
        const MNT_QUOTA = 0x0000_2000;
        /// Identifies the root filesystem
        const MNT_ROOTFS = 0x0000_4000;
        /// FS supports `volfs` (deprecated flag in Mac OS X 10.5)
        const MNT_DOVOLFS = 0x0000_8000;

        /// File system is not appropriate path to user data
        const MNT_DONTBROWSE = 0x0010_0000;
        /// VFS will ignore ownership information on filesystem objects
        const MNT_IGNORE_OWNERSHIP = 0x0020_0000;
        /// Filesystem was mounted by automounter
        const MNT_AUTOMOUNTED = 0x0040_0000;
        /// Filesystem is journaled
        const MNT_JOURNALED = 0x0080_0000;
        /// Don't allow user extended attributes
        const MNT_NOUSERXATTR = 0x0100_0000;
        /// Filesystem should defer writes
        const MNT_DEFWRITE = 0x0200_0000;
        /// MAC support for individual labels
        const MNT_MULTILABEL = 0x0400_0000;
        /// Disable update of file access time
        const MNT_NOATIME = 0x1000_0000;

        // External filesystem command modifier flags

        /// Not a real mount, just an update
        const MNT_UPDATE = 0x0001_0000;
        /// Don't block unmount if not responding
        const MNT_NOBLOCK = 0x0002_0000;
        /// Reload filesystem data
        const MNT_RELOAD = 0x0004_0000;
        /// Force unmount or readonly change
        const MNT_FORCE = 0x0008_0000;
        // `bitflags` at this point is not allowing to use other constants
        //                   MNT_UPDATE | MNT_NOBLOCK | MNT_RELOAD | MNT_FORCE
        const MNT_CMDFLAGS = 0x0001_0000 | 0x0002_0000 | 0x0004_0000 | 0x0008_0000;
    }
}

impl Flags {
    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn into_string(self) -> String {
        let mut buffer: Vec<&'static str> = Vec::with_capacity(self.bits.count_ones() as usize);

        if self.contains(Self::MNT_RDONLY) {
            buffer.push("ro")
        } else {
            buffer.push("rw")
        }

        if self.contains(Self::MNT_SYNCHRONOUS) {
            buffer.push("sync");
        }
        if self.contains(Self::MNT_NOEXEC) {
            buffer.push("noexec");
        }
        if self.contains(Self::MNT_NOSUID) {
            buffer.push("nosuid");
        }
        if self.contains(Self::MNT_NODEV) {
            buffer.push("nodev");
        }
        if self.contains(Self::MNT_UNION) {
            buffer.push("union");
        }
        if self.contains(Self::MNT_ASYNC) {
            buffer.push("async");
        }
        if self.contains(Self::MNT_CPROTECT) {
            // TODO: Not sure if properly named option
            buffer.push("cprotect");
        }
        if self.contains(Self::MNT_EXPORTED) {
            buffer.push("exported");
        }
        if self.contains(Self::MNT_QUARANTINE) {
            buffer.push("quarantine");
        }
        if self.contains(Self::MNT_LOCAL) {
            buffer.push("local");
        }
        if self.contains(Self::MNT_QUOTA) {
            buffer.push("quota");
        }
        if self.contains(Self::MNT_ROOTFS) {
            buffer.push("rootfs");
        }
        if self.contains(Self::MNT_DOVOLFS) {
            buffer.push("dovolfs");
        }
        if self.contains(Self::MNT_DONTBROWSE) {
            buffer.push("dontbrowse");
        }
        if self.contains(Self::MNT_IGNORE_OWNERSHIP) {
            buffer.push("ignore-ownership");
        }
        if self.contains(Self::MNT_AUTOMOUNTED) {
            buffer.push("automounted");
        }
        if self.contains(Self::MNT_JOURNALED) {
            buffer.push("journaled");
        }
        if self.contains(Self::MNT_NOUSERXATTR) {
            buffer.push("nouserxattr");
        }
        if self.contains(Self::MNT_DEFWRITE) {
            buffer.push("defwrite");
        }
        if self.contains(Self::MNT_MULTILABEL) {
            buffer.push("multilabel");
        }
        if self.contains(Self::MNT_NOATIME) {
            buffer.push("noatime");
        }
        if self.contains(Self::MNT_UPDATE) {
            buffer.push("update");
        }
        if self.contains(Self::MNT_RELOAD) {
            buffer.push("reload");
        }
        if self.contains(Self::MNT_FORCE) {
            buffer.push("force");
        }
        if self.contains(Self::MNT_CMDFLAGS) {
            buffer.push("cmdflags");
        }

        buffer.join(",")
    }
}

pub trait PartitionExt {
    fn flags(&self) -> Flags;
}

#[cfg(target_os = "macos")]
impl PartitionExt for crate::Partition {
    fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.as_ref().raw_flags())
    }
}
