use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::ffi::{OsStrExt, OsStringExt};

use heim_runtime as rt;

use crate::sys::linux::process::procfs::process_file_path;
use crate::{Pid, ProcessError, ProcessResult};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Delimiter {
    Null,
    Space,
}

impl Delimiter {
    fn as_char(self) -> char {
        match self {
            Delimiter::Null => '\0',
            Delimiter::Space => ' ',
        }
    }
}

impl From<char> for Delimiter {
    // `man proc` says that delimiter between parts is the `\0`,
    // but some programs are using ' ' (ASCII space).
    //
    // And if there some bad boy over there,
    // falling back to `\0`, just in case.
    fn from(raw: char) -> Delimiter {
        match raw {
            '\0' => Delimiter::Null,
            ' ' => Delimiter::Space,
            _ => Delimiter::Null,
        }
    }
}

impl From<u8> for Delimiter {
    fn from(raw: u8) -> Delimiter {
        match raw {
            b'\0' => Delimiter::Null,
            b' ' => Delimiter::Space,
            _ => Delimiter::Null,
        }
    }
}

#[derive(Debug)]
pub struct Command {
    line: OsString,
    delimiter: Delimiter,
}

impl Command {
    pub fn to_os_string(&self) -> OsString {
        let line = self.line.clone();

        match self.delimiter {
            Delimiter::Space => line,
            Delimiter::Null => Self::with_spaces(line),
        }
    }

    pub fn into_os_string(self) -> OsString {
        match self.delimiter {
            Delimiter::Space => self.line,
            Delimiter::Null => Self::with_spaces(self.line),
        }
    }

    fn with_spaces(line: OsString) -> OsString {
        let mut bytes = line.into_vec();
        for byte in bytes.iter_mut() {
            if *byte == b'\0' {
                *byte = b' ';
            }
        }
        // Dropping trailing delimiter
        let _ = bytes.pop();

        OsString::from_vec(bytes)
    }
}

impl<T> From<T> for Command
where
    T: Into<OsString>,
{
    fn from(os_string: T) -> Command {
        let os_string = os_string.into();

        let delimiter = match os_string.as_bytes().last() {
            Some(chr) => Delimiter::from(*chr),
            None => Delimiter::Null,
        };

        Command {
            line: os_string,
            delimiter,
        }
    }
}

impl<'a> IntoIterator for &'a Command {
    type Item = &'a OsStr;
    type IntoIter = CommandIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CommandIter {
            line: self.line.as_os_str(),
            delimiter: self.delimiter,
            position: 0,
        }
    }
}

#[derive(Debug)]
pub struct CommandIter<'a> {
    line: &'a OsStr,
    delimiter: Delimiter,
    position: usize,
}

impl<'a> Iterator for CommandIter<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.line.len() {
            return None;
        }

        let bytes = &self.line.as_bytes()[self.position..];
        match memchr::memchr(self.delimiter.as_char() as u8, bytes) {
            Some(offset) => {
                let slice = &bytes[..offset];
                // `+ 1` is for skipping the trailing delimiter of this argument slice
                self.position += offset + 1;

                Some(OsStr::from_bytes(slice))
            }
            None => None,
        }
    }
}

pub async fn command(pid: Pid) -> ProcessResult<Command> {
    match rt::fs::read_to_string(process_file_path(pid, "cmdline")).await {
        Ok(contents) => Ok(Command::from(contents)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Err(ProcessError::NoSuchProcess(pid)),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::{OsStr, OsString};

    use super::Command;

    #[test]
    fn test_iter_with_nulls() {
        let line = OsString::from("/usr/bin/ntpd\0-g\0-u\0ntp:ntp\0");
        let command = Command::from(line);
        let iter = &mut command.into_iter();

        assert_eq!(Some(OsStr::new("/usr/bin/ntpd")), iter.next());
        assert_eq!(Some(OsStr::new("-g")), iter.next());
        assert_eq!(Some(OsStr::new("-u")), iter.next());
        assert_eq!(Some(OsStr::new("ntp:ntp")), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_iter_with_spaces() {
        let line = OsString::from("/opt/atom/atom --type=renderer --no-sandbox --lang=en-US ");
        let command = Command::from(line);
        let iter = &mut command.into_iter();

        assert_eq!(Some(OsStr::new("/opt/atom/atom")), iter.next());
        assert_eq!(Some(OsStr::new("--type=renderer")), iter.next());
        assert_eq!(Some(OsStr::new("--no-sandbox")), iter.next());
        assert_eq!(Some(OsStr::new("--lang=en-US")), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_iter_empty() {
        let command = Command::from("");
        let iter = &mut command.into_iter();

        assert_eq!(None, iter.next());
    }
}
