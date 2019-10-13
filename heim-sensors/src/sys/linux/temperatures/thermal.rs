use std::ffi::OsStr;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
use heim_common::units::ThermodynamicTemperature;
use heim_runtime::fs;

use crate::TemperatureSensor;

use super::sysfs::{self, read_temperature};

#[derive(Debug)]
enum TripPoint {
    High(ThermodynamicTemperature),
    Critical(ThermodynamicTemperature),
}

/// For received `/sys/class/thermal/*/trip_point_0_type` dir entry
/// returns its path and the `*/trip_point_0_temp` path.
fn trip_point_paths(entry: fs::DirEntry) -> Result2<Option<(PathBuf, PathBuf)>> {
    let file_name = entry.file_name();
    let bytes = file_name.as_bytes();
    if !(bytes.starts_with(b"trip_point_") && bytes.ends_with(b"_type")) {
        return Ok(None);
    }

    const PREFIX_LEN: usize = b"trip_point_".len();
    const POSTFIX_LEN: usize = b"_type".len();

    let path = entry.path();
    let raw_idx = OsStr::from_bytes(&bytes[PREFIX_LEN..bytes.len() - POSTFIX_LEN]);
    let idx_str = raw_idx
        .to_str()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
    let idx = idx_str.parse::<u32>()?;

    let parent = path.parent().unwrap_or_else(|| {
        unreachable!("Directory traversal at `thermal_zone` should guarantee that");
    });
    let temp_path = parent.join(format!("trip_point_{}_temp", idx));

    Ok(Some((temp_path, path)))
}

async fn read_trip_point(entry: fs::DirEntry) -> Result2<Option<TripPoint>> {
    let (temp_path, type_path) = match trip_point_paths(entry)? {
        Some(paths) => paths,
        None => return Ok(None),
    };

    let temperature = read_temperature(temp_path).await?;
    let point_type = fs::read_to_string(type_path).await?;

    match point_type.as_str() {
        "critical\n" => Ok(Some(TripPoint::Critical(temperature))),
        "high\n" => Ok(Some(TripPoint::High(temperature))),
        _ => Err(io::Error::from(io::ErrorKind::InvalidData).into()),
    }
}

async fn read_sensor(entry: fs::DirEntry) -> Result2<TemperatureSensor> {
    let root = entry.path();
    let temperature = sysfs::read_temperature(root.join("temp")).await?;
    let unit_name = sysfs::read_string(root.join("type")).await?;
    let mut sensor = TemperatureSensor {
        unit: unit_name,
        label: None,
        current: temperature,
        high: None,
        critical: None,
    };

    let mut entries = fs::read_dir(root).await?;
    while let Some(try_entry) = entries.next().await {
        match try_entry {
            Ok(entry) => match read_trip_point(entry).await {
                Ok(Some(TripPoint::High(temp))) => {
                    sensor.high = Some(temp);
                }
                Ok(Some(TripPoint::Critical(temp))) => {
                    sensor.critical = Some(temp);
                }
                _ => continue,
            },
            Err(..) => continue,
        }
    }

    Ok(sensor)
}

// https://www.kernel.org/doc/Documentation/thermal/sysfs-api.txt
pub fn thermal_zone<T>(root: T) -> impl Stream<Item = Result2<TemperatureSensor>>
where
    T: AsRef<Path>,
{
    fs::read_dir(root)
        .try_flatten_stream()
        .try_filter(|entry| {
            future::ready(entry.file_name().as_bytes().starts_with(b"thermal_zone"))
        })
        .map_err(Error2::from)
        .and_then(read_sensor)
}
