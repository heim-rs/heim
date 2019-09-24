use std::convert::TryInto;
use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::ffi::{OsStrExt, OsStringExt};

use crate::sys::macos::bindings;
use crate::Pid;

/// Somewhat idiomatic wrapper for the data returned by a `KERN_PROCARGS2` syscall.
///
/// Does NO validation on the input!
#[derive(Debug)]
pub struct ProcArgs(Vec<u8>);

impl ProcArgs {
    pub fn get(pid: Pid) -> io::Result<Self> {
        bindings::proc_args(pid).map(Self)
    }

    pub fn argc(&self) -> u32 {
        debug_assert!(self.0.len() >= 4);

        u32::from_ne_bytes(self.0[..4].try_into().expect("Malformed procargs array"))
    }

    #[allow(dead_code)]
    pub fn exe(&self) -> &OsStr {
        let (start, end) = self.exe_range();
        OsStr::from_bytes(&self.0[start..end])
    }

    pub fn arguments(&self) -> ProcArgsArguments<'_> {
        let (start, end) = self.arguments_range();
        ProcArgsArguments {
            left: self.argc(),
            data: &self.0[start..end],
        }
    }

    /// Returns absolute `(start, end)` positions for the executable path
    fn exe_range(&self) -> (usize, usize) {
        (
            4, // argc length
            memchr::memchr(b'\0', &self.0[4..]).unwrap_or(0) + 4,
        )
    }

    fn arguments_range(&self) -> (usize, usize) {
        let argc = self.argc() as usize;
        let (_, exe_end) = self.exe_range();
        // after the `exe_end` index there might be some more null bytes (1+),
        // we should skip them too and make this value absolute
        // (meaning it would start from the zero index of `self.0`
        let start = self
            .0
            .iter()
            .skip(exe_end)
            .position(|byte| *byte != b'\0')
            .unwrap_or(0)
            + exe_end;

        // end index should be absolute too and also exclude the trailing `\0`
        let end = memchr::memchr_iter(b'\0', &self.0[start..])
            .nth(argc - 1)
            .unwrap_or(0)
            + start;

        // Converting from relative to absolute offsets
        (start, end)
    }

    /// Grab the unparsed `arguments`, replace `'\0'` with `' '` and return it
    pub fn to_command(&self) -> OsString {
        let (start, end) = self.arguments_range();
        let mut bytes = Vec::from(&self.0[start..end]);
        for byte in bytes.iter_mut() {
            if *byte == b'\0' {
                *byte = b' ';
            }
        }

        OsString::from_vec(bytes)
    }
}

#[derive(Debug)]
pub struct ProcArgsArguments<'a> {
    left: u32,
    data: &'a [u8],
}

impl<'a> Iterator for ProcArgsArguments<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            return None;
        }

        self.left -= 1;
        match memchr::memchr(b'\0', self.data) {
            Some(end) => {
                let arg = &self.data[..end];
                // `+ 1` is for `\0` delimiter, which should be skipped
                self.data = &self.data[(end + 1)..];

                Some(arg)
            }
            None => {
                // End of the arguments, popping up the last one
                Some(&self.data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    use super::ProcArgs;

    static EXAMPLE: &[u8] = b"\x04\0\0\0./process_current\0\0\0\0\0\0\0\
./process_current\0-a\0-b\0--co=2\0\
TERM=xterm-256color\0SHELL=/bin/bash\0PWD=/\0_=./process_current\0\0\0\0\0\0\0\0\
\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
main_stack=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
\0\0\0\0\0\0\0\0\0\0\0\0\0\0\
executable_file=0x1000003,0xb66dc\0dyld_file=0x1000003,0x62a1e\0\0\0\0\0\0\0\0";

    #[test]
    fn test_proc_args_count() {
        let proc = ProcArgs(Vec::from(EXAMPLE));

        assert_eq!(proc.argc(), 4);
    }

    #[test]
    fn test_proc_args_exe_range() {
        let proc = ProcArgs(Vec::from(EXAMPLE));

        assert_eq!((4, 21), proc.exe_range());
    }

    #[test]
    fn test_proc_args_exe() {
        let proc = ProcArgs(Vec::from(EXAMPLE));

        assert_eq!(OsStr::from_bytes(b"./process_current"), proc.exe());
    }

    #[test]
    fn test_proc_args_to_command() {
        let proc = ProcArgs(Vec::from(EXAMPLE));

        assert_eq!(
            OsStr::from_bytes(b"./process_current -a -b --co=2"),
            &proc.to_command()
        );
    }

    #[test]
    fn test_proc_args_arguments_range() {
        let proc = ProcArgs(Vec::from(EXAMPLE));

        assert_eq!((28, 58), proc.arguments_range());
    }

    #[test]
    fn test_proc_args_arguments() {
        let proc = ProcArgs(Vec::from(EXAMPLE));
        let mut args = proc.arguments();

        assert_eq!(Some(&b"./process_current"[..]), args.next());
        assert_eq!(Some(&b"-a"[..]), args.next());
        assert_eq!(Some(&b"-b"[..]), args.next());
        assert_eq!(Some(&b"--co=2"[..]), args.next());
        assert_eq!(None, args.next());
    }
}
