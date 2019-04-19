bitflags::bitflags! {
    /// Various options that were employed when mounting this filesystem (see `statvfs(3)`).
    ///
    /// See [UsageExt::flags] method, which returns that structure.
    pub struct Flags: libc::c_ulong {
        /// Mandatory locking is permitted on the filesystem (see `fcntl(2)`).
        ///
        /// ## Compatibility
        ///
        /// Is not defined for MacOS
        #[cfg(any(doc, not(target_os = "macos")))]
        const MANDLOCK = libc::ST_MANDLOCK;

        /// Do not update access times; see `mount(2)`.
        ///
        /// ## Compatibility
        ///
        /// Is not defined for MacOS
        #[cfg(any(doc, not(target_os = "macos")))]
        const NOATIME = libc::ST_NOATIME;

        /// Disallow access to device special files on this filesystem.
        ///
        /// ## Compatibility
        ///
        /// Is not defined for MacOS
        #[cfg(any(doc, not(target_os = "macos")))]
        const NODEV = libc::ST_NODEV;

        /// Do not update directory access times; see `mount(2)`.
        ///
        /// ## Compatibility
        ///
        /// Is not defined for MacOS
        #[cfg(any(doc, not(target_os = "macos")))]
        const NODIRATIME = libc::ST_NODIRATIME;

        /// Execution of programs is disallowed on this filesystem.
        ///
        /// ## Compatibility
        ///
        /// Is not defined for MacOS
        #[cfg(any(doc, not(target_os = "macos")))]
        const NOEXEC = libc::ST_NOEXEC;

        /// The set-user-ID and set-group-ID bits are ignored by `exec(3)`
        /// for executable files on this filesystem.
        const NOSUID = libc::ST_NOSUID;

        /// This filesystem is mounted read-only.
        const RDONLY = libc::ST_RDONLY;

        /// Update `atime` relative to `mtime`/`ctime`; see `mount(2)`.
        ///
        /// ## Compatibility
        ///
        /// Is not defined for MacOS
        #[cfg(any(doc, not(target_os = "macos")))]
        const RELATIME = libc::ST_RELATIME;

        /// Writes are synced to the filesystem immediately
        /// (see the description of `O_SYNC` in `open(2)`).
        ///
        /// ## Compatibility
        ///
        /// Is not defined for MacOS
        #[cfg(any(doc, not(target_os = "macos")))]
        const SYNCHRONOUS = libc::ST_SYNCHRONOUS;
    }
}

#[heim_derive::os_ext_for(crate::Usage, cfg(unix))]
pub trait UsageExt {
    /// Returns [Flags] for current filesystem;
    fn flags(&self) -> Flags;
}
