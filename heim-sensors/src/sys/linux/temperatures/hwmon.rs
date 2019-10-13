use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use heim_common::prelude::*;
use heim_runtime::fs;

use super::sysfs;
use crate::TemperatureSensor;

pub fn hwmon<T>(path: T) -> impl Stream<Item = Result2<TemperatureSensor>>
where
    T: AsRef<Path>,
{
    fs::read_dir(path)
        .try_flatten_stream()
        .try_filter_map(read_subfolder)
        // At this point we have a stream of all files
        // matchable to the `/sys/class/hwmon/hwmon([0-9]+)/*`
        .try_flatten()
        // But we need only the `temp([0-9]+)_input`
        .try_filter(|entry| {
            let file_name = entry.file_name();
            let bytes = file_name.as_bytes();

            future::ready(bytes.starts_with(b"temp") && bytes.ends_with(b"_input"))
        })
        .map_err(Error2::from)
        .and_then(read_sensor)
}

async fn read_subfolder(
    entry: fs::DirEntry,
) -> io::Result<Option<impl Stream<Item = io::Result<fs::DirEntry>>>> {
    let file_name = entry.file_name();
    // Should not be a case, but skipping all the folders,
    // which are not matchable to `/sys/class/hwmon/hwmon([0-9]+)` pattern
    if !file_name.as_bytes().starts_with(b"hwmon") {
        return Ok(None);
    }

    let root = entry.path();

    // CentOS has an intermediate `device/` directory:
    // https://github.com/giampaolo/psutil/issues/971
    // https://github.com/nicolargo/glances/issues/1060
    let intermediate_folder = root.join("device");
    let root = if fs::path_exists(&intermediate_folder).await {
        intermediate_folder
    } else {
        root
    };

    Ok(Some(fs::read_dir(root).await?))
}

/// Read whole data for the temperature sensor.
///
/// `entry` is pointing to the `temp*_input` file.
async fn read_sensor(entry: fs::DirEntry) -> Result2<TemperatureSensor> {
    let input_path = entry.path();
    // Safety: directory traversal guarantees that there will be a parent directory
    let root = input_path.parent().unwrap_or_else(|| unreachable!());

    let name_path = root.join("name");
    let label_path = sysfs::replace_postfix(&input_path, b"input", b"label");
    let max_path = sysfs::replace_postfix(&input_path, b"input", b"max");
    let crit_path = sysfs::replace_postfix(&input_path, b"input", b"crit");

    // In a previous iteration this code was using `future::try_join5`,
    // which effectively loaded all 5 files concurrently.
    // As with the current nightly build (2019-10-13) it generates a crazy big type as a result,
    // which overflows default Rust type size limit,
    // concurrent loading was removed in favor of plain non-parallel IO.
    //
    // TODO: use concurrent loading for data as soon as `async_await` will be stable

    Ok(TemperatureSensor {
        unit: sysfs::read_string(name_path).await?,
        label: sysfs::read_string(label_path).await.ok(),
        current: sysfs::read_temperature(input_path).await?,
        high: sysfs::read_temperature(max_path).await.ok(),
        critical: sysfs::read_temperature(crit_path).await.ok(),
    })
}
