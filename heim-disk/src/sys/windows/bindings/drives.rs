use std::ptr;

use heim_common::{Error, Result};
use winapi::ctypes::wchar_t;
use winapi::um::fileapi;

use super::Drive;

/// Iterator over valid drives (ex. `A:\`, `C:\`) in system.
pub struct Drives {
    buffer: Vec<wchar_t>,
    position: usize,
}

impl Drives {
    pub fn new() -> Result<Drives> {
        let expected_size = unsafe { fileapi::GetLogicalDriveStringsW(0, ptr::null_mut()) };

        if expected_size == 0 {
            return Err(Error::last_os_error());
        }

        let mut buffer = Vec::with_capacity(expected_size as usize);
        let result =
            unsafe { fileapi::GetLogicalDriveStringsW(expected_size, buffer.as_mut_ptr()) };

        if result == 0 {
            return Err(Error::last_os_error());
        }

        // the return value is the length, in characters, of the strings copied to the buffer,
        // not including the terminating null character.
        debug_assert!(expected_size == result + 1);
        unsafe {
            buffer.set_len((result + 1) as usize);
        }

        Ok(Drives {
            buffer,
            position: 0,
        })
    }
}

impl Iterator for Drives {
    type Item = Drive;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;

        match self
            .buffer
            .iter()
            .skip(position)
            .position(|chr| *chr == 0x00)
        {
            Some(offset) if offset > 0 => {
                // We are going to include trailing \0 too
                let end = position + offset + 1;

                self.position = end;
                Some(Drive::from(&self.buffer[position..end]))
            }
            _ => None,
        }
    }
}

impl From<Vec<wchar_t>> for Drives {
    fn from(buffer: Vec<wchar_t>) -> Drives {
        Drives {
            buffer,
            position: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Drives;

    #[test]
    fn test_empty_iterator() {
        let buffer = vec![0x0000];
        let drives = Drives::from(buffer);

        assert_eq!(drives.count(), 0);
    }

    #[test]
    fn test_iterator() {
        // A:\<nul>B:\<nul>C:\<nul><nul>
        let buffer = vec![
            0x0041, 0x003a, 0x005c, 0x0000, // A:\
            0x0043, 0x003a, 0x005c, 0x0000, // C:\
            0x0044, 0x003a, 0x005c, 0x0000, // D:\
            0x0000,
        ];
        let drives = Drives::from(buffer);

        assert_eq!(drives.count(), 3);
    }
}
