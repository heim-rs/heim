use std::fmt;
use std::vec;
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

pub struct EnvOs {
    inner: vec::IntoIter<(OsString, OsString)>,
}

impl EnvOs {
    pub fn from_bytes(bytes: &[u8]) -> EnvOs {
        let inner = bytes.split(|byte| *byte == b'\0')
            .filter_map(|kv| {
                let mut parts = kv.splitn(2, |byte| *byte == b'=');

                let key = match parts.next() {
                    Some(bytes) => OsStr::from_bytes(bytes).to_os_string(),
                    None => return None,
                };

                let value = match parts.next() {
                    Some(bytes) => OsStr::from_bytes(bytes).to_os_string(),
                    None => return None,
                };

                Some((key, value))
            })
            .collect::<Vec<_>>()
            .into_iter();

        EnvOs {
            inner,
        }
    }
}

impl Iterator for EnvOs {
    type Item = (OsString, OsString);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl fmt::Debug for EnvOs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("EnvOs { .. }")
    }
}
