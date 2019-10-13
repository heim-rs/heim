use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
use heim_common::units::{thermodynamic_temperature, ThermodynamicTemperature};
use heim_runtime::fs;

/// Creates new `PathBuf` from `path` with `old` postfix replaced with `new`.
pub fn replace_postfix(path: &Path, old: &[u8], new: &[u8]) -> PathBuf {
    // In the `heim` case all the `new` postfixes has at most the same length as the `old` has,
    // so futher `buffer.push` should not result in the re-allocations
    // and we could skip the check if we need bigger buffer.
    //
    // If it will be changed later, we will hit a small performance degradation,
    // but the code will work as expected, so it is a reasonable trade-off.

    let mut buffer = OsString::with_capacity(path.as_os_str().len());
    let bytes = path.as_os_str().as_bytes();
    buffer.push(OsStr::from_bytes(&bytes[..bytes.len() - old.len()]));
    buffer.push(OsStr::from_bytes(new));

    PathBuf::from(buffer)
}

pub async fn read_temperature(path: PathBuf) -> Result<ThermodynamicTemperature> {
    let contents = fs::read_to_string(path).await?;
    // Originally value is in millidegrees of Celsius
    let value = contents.trim_end().parse::<f32>()? / 1_000.0;

    Ok(ThermodynamicTemperature::new::<
        thermodynamic_temperature::degree_celsius,
    >(value))
}

pub async fn read_string(path: PathBuf) -> Result<String> {
    let mut contents = fs::read_to_string(path).await?;
    // In-place trimming, as we know that it ends with `\n`
    let _ = contents.pop();

    Ok(contents)
}
