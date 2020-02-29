//! Unix-alike process environment parser.

use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStringExt;
use std::slice;
use std::vec;

/// Container for environment variables.
#[derive(Debug)]
pub struct Environment(Vec<(OsString, OsString)>);

impl Environment {
    /// Parse environment variables from the `bytes` blob, separated by `\0`.
    ///
    /// Reference: https://github.com/rust-lang/rust/blob/04e7f96dd89b1f0ad615dff1c85d11d4c4c64cb4/src/libstd/sys/unix/os.rs#L494-L510
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let capacity = memchr::memchr_iter(b'\0', bytes).count();
        let mut result = Vec::with_capacity(capacity);
        let mut position = 0;

        while let Some(offset) = memchr::memchr(b'\0', &bytes[position..]) {
            // Null byte at the start or the double null byte means that we are also done parsing
            if offset == 0 {
                break;
            }
            let kv = &bytes[position..position + offset];

            // Skipping the trailing `\0` also
            position += offset + 1;

            match memchr::memchr(b'=', &kv[1..]) {
                Some(mut delimiter) => {
                    // Skipping the `=` part
                    delimiter += 1;
                    let key = OsString::from_vec(kv[..delimiter].to_vec());
                    let value = OsString::from_vec(kv[delimiter + 1..].to_vec());

                    result.push((key, value));
                }
                None => continue,
            }
        }

        Self(result)
    }

    pub fn iter(&self) -> EnvironmentIter {
        let kv = self.0.iter();

        EnvironmentIter(kv)
    }
}

impl IntoIterator for Environment {
    type Item = (OsString, OsString);
    type IntoIter = IntoEnvironmentIter;

    fn into_iter(self) -> Self::IntoIter {
        let kv = self.0.into_iter();

        IntoEnvironmentIter(kv)
    }
}

impl<'e> IntoIterator for &'e Environment {
    type Item = (&'e OsStr, &'e OsStr);
    type IntoIter = EnvironmentIter<'e>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct IntoEnvironmentIter(vec::IntoIter<(OsString, OsString)>);

impl Iterator for IntoEnvironmentIter {
    type Item = (OsString, OsString);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

#[derive(Debug)]
pub struct EnvironmentIter<'e>(slice::Iter<'e, (OsString, OsString)>);

impl<'e> Iterator for EnvironmentIter<'e> {
    type Item = (&'e OsStr, &'e OsStr);

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some((k, v)) => Some((k.as_os_str(), v.as_os_str())),
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    use super::Environment;

    static SUITE: &[u8] = b"COLORFGBG=15;0\0COLORTERM=truecolor\0DISPLAY=:0\0HOME=/home/user\0LANG=en_US.UTF-8\0MAIL=/var/spool/mail/user\0PROFILEHOME=\0PATH=/home/user/.cargo/bin:/usr/local/bin:/usr/bin\0";

    #[test]
    fn test_iter() {
        let mut env = Environment::from_bytes(&SUITE).into_iter();

        assert_eq!(
            Some((
                OsStr::from_bytes(b"COLORFGBG").into(),
                OsStr::from_bytes(b"15;0").into()
            )),
            env.next(),
        );
        assert_eq!(
            Some((
                OsStr::from_bytes(b"COLORTERM").into(),
                OsStr::from_bytes(b"truecolor").into()
            )),
            env.next(),
        );
        assert_eq!(
            Some((
                OsStr::from_bytes(b"DISPLAY").into(),
                OsStr::from_bytes(b":0").into()
            )),
            env.next(),
        );
        assert_eq!(
            Some((
                OsStr::from_bytes(b"HOME").into(),
                OsStr::from_bytes(b"/home/user").into()
            )),
            env.next(),
        );
        assert_eq!(
            Some((
                OsStr::from_bytes(b"LANG").into(),
                OsStr::from_bytes(b"en_US.UTF-8").into()
            )),
            env.next(),
        );
        assert_eq!(
            Some((
                OsStr::from_bytes(b"MAIL").into(),
                OsStr::from_bytes(b"/var/spool/mail/user").into()
            )),
            env.next(),
        );
        assert_eq!(
            Some((
                OsStr::from_bytes(b"PROFILEHOME").into(),
                OsStr::from_bytes(b"").into()
            )),
            env.next(),
        );
        assert_eq!(
            Some((
                OsStr::from_bytes(b"PATH").into(),
                OsStr::from_bytes(b"/home/user/.cargo/bin:/usr/local/bin:/usr/bin").into()
            )),
            env.next(),
        );
        assert_eq!(None, env.next());
    }

    #[test]
    fn test_empty() {
        let mut env = Environment::from_bytes(b"").into_iter();
        assert_eq!(None, env.next());
    }

    #[test]
    fn test_nulls() {
        let mut env =
            Environment::from_bytes(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
                .into_iter();
        assert_eq!(None, env.next());
    }
}
