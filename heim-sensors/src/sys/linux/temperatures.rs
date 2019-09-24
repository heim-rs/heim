use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use heim_common::prelude::*;
use heim_common::units::{thermodynamic_temperature, ThermodynamicTemperature};
use heim_common::utils::stream::HeimStreamExt;
use heim_runtime::fs;

use crate::TemperatureSensor;

#[inline]
fn file_name(prefix: &OsStr, postfix: &[u8]) -> OsString {
    let mut name = OsString::with_capacity(prefix.len() + postfix.len());
    name.push(prefix);
    name.push(OsStr::from_bytes(postfix));

    name
}

fn read_temperature(path: PathBuf) -> impl Future<Output = Result<ThermodynamicTemperature>> {
    fs::read_to_string(path)
        .map_err(Error::from)
        .and_then(|contents| {
            match contents.trim_end().parse::<f32>() {
                // Originally value is in millidegrees of Celsius
                Ok(value) => future::ok(ThermodynamicTemperature::new::<
                    thermodynamic_temperature::degree_celsius,
                >(value / 1_000.0)),
                Err(e) => future::err(e.into()),
            }
        })
}

fn hwmon_sensor(input: PathBuf) -> impl Future<Output = Result<TemperatureSensor>> {
    // It is guaranteed by `hwmon` and `hwmon_sensor` directory traversals,
    // that it is not a root directory and it points to a file.
    // Otherwise it is an implementation bug.
    let root = input.parent().unwrap_or_else(|| unreachable!());
    let prefix = match input.file_name() {
        Some(name) => {
            let offset = name.len() - b"input".len();
            OsStr::from_bytes(&name.as_bytes()[..offset])
        }
        None => unreachable!(),
    };

    let unit_name = fs::read_to_string(root.join("name"))
        .map_err(Error::from)
        .map_ok(|mut string| {
            // Dropping trailing `\n`
            let _ = string.pop();
            string
        });
    let label = fs::read_to_string(root.join(file_name(prefix, b"label")))
        .map_err(Error::from)
        .map_ok(|mut string| {
            // Dropping trailing `\n`
            let _ = string.pop();
            Some(string)
        })
        .or_else(|_e| {
            // TODO: Would it be reasonable to propagate errors other than NotFound?
            future::ok::<_, Error>(None)
        });
    let high = read_temperature(root.join(file_name(prefix, b"max")))
        .map_ok(Some)
        .or_else(|_e| {
            // TODO: Would it be reasonable to propagate errors other than NotFound?
            future::ok::<_, Error>(None)
        });
    let critical = read_temperature(root.join(file_name(prefix, b"crit")))
        .map_ok(Some)
        .or_else(|_e| {
            // TODO: Would it be reasonable to propagate errors other than NotFound?
            future::ok::<_, Error>(None)
        });
    let current = read_temperature(input);

    future::try_join5(unit_name, label, current, high, critical).map_ok(
        |(unit, label, current, high, critical)| TemperatureSensor {
            unit,
            label,
            current,
            high,
            critical,
        },
    )
}

fn hwmon() -> impl Stream<Item = Result<TemperatureSensor>> {
    // TODO: It would be nice to have async glob matchers :(
    // Basically we are searching for `/sys/class/hwmon/temp*_*` files here
    fs::read_dir("/sys/class/hwmon/")
        .try_filter(|entry| future::ready(entry.file_name().as_bytes().starts_with(b"hwmon")))
        .and_then(|entry| {
            let inner = fs::read_dir(entry.path()).try_filter(|entry| {
                let name = entry.file_name();
                let bytes = name.as_bytes();

                future::ready(bytes.starts_with(b"temp") && bytes.ends_with(b"_input"))
            });

            future::ok(inner)
        })
        .try_flatten()
        .map_err(Error::from)
        .and_then(|entry| hwmon_sensor(entry.path()))
}

// CentOS has an intermediate /device directory:
// https://github.com/giampaolo/psutil/issues/971
// https://github.com/nicolargo/glances/issues/1060
fn hwmon_device() -> impl Stream<Item = Result<TemperatureSensor>> {
    // TODO: It would be nice to have async glob matchers :(
    // Basically we are searching for `/sys/class/hwmon/temp*_*` files here
    fs::read_dir("/sys/class/hwmon/")
        .try_filter(|entry| future::ready(entry.file_name().as_bytes().starts_with(b"hwmon")))
        .try_filter(|entry| {
            // TODO: `entry.path()` allocates memory for `PathBuf` twice
            // here and in the next combinator
            fs::path_exists(entry.path().join("device"))
        })
        .and_then(|entry| {
            let inner = fs::read_dir(entry.path().join("device")).try_filter(|entry| {
                let name = entry.file_name();
                let bytes = name.as_bytes();

                future::ready(bytes.starts_with(b"temp") && bytes.ends_with(b"_input"))
            });

            future::ok(inner)
        })
        .try_flatten()
        .map_err(Error::from)
        .and_then(|entry| hwmon_sensor(entry.path()))
}

// https://www.kernel.org/doc/Documentation/thermal/sysfs-api.txt
fn thermal_zone() -> impl Stream<Item = Result<TemperatureSensor>> {
    fs::read_dir("/sys/class/thermal/")
        .try_filter(|entry| {
            future::ready(entry.file_name().as_bytes().starts_with(b"thermal_zone"))
        })
        .map_err(Error::from)
        .and_then(|entry| {
            let root = entry.path();
            let temperature = read_temperature(root.join("temp"));
            let unit_name = fs::read_to_string(root.join("type"))
                .map_err(Error::from)
                .map_ok(|mut string| {
                    // Dropping trailing `\n`
                    let _ = string.pop();
                    string
                });

            future::try_join(temperature, unit_name).map_ok(|(temp, unit)| (root, temp, unit))
        })
        .and_then(|(root, temp, unit)| {
            let sensor = TemperatureSensor {
                unit,
                label: None,
                current: temp,
                high: None,
                critical: None,
            };

            fs::read_dir(root)
                .try_filter(|entry| {
                    let name = entry.file_name();
                    let bytes = name.as_bytes();

                    future::ready(bytes.starts_with(b"trip_point_") && bytes.ends_with(b"type"))
                })
                .map_err(Error::from)
                .try_fold(sensor, |mut acc, entry| {
                    let name = entry.file_name();
                    let offset = name.len() - b"type".len();
                    let prefix = OsStr::from_bytes(&name.as_bytes()[..offset]);

                    let type_path = entry.path();
                    let root = type_path.parent().unwrap_or_else(|| unreachable!());
                    let temp_path = root.join(file_name(prefix, b"temp"));

                    // TODO: Rewrite with `async_await` when it will be stable
                    // Because right now it looks just terrible
                    fs::read_to_string(type_path)
                        .map_err(Error::from)
                        .and_then(move |content| match content.as_str() {
                            "critical\n" => read_temperature(temp_path)
                                .and_then(move |temp| {
                                    acc.critical = Some(temp);
                                    future::ok(acc)
                                })
                                .boxed(),
                            "high\n" => read_temperature(temp_path)
                                .and_then(move |temp| {
                                    acc.high = Some(temp);
                                    future::ok(acc)
                                })
                                .boxed(),
                            _ => future::ok(acc).boxed(),
                        })
                })
        })
}

pub fn temperatures() -> impl Stream<Item = Result<TemperatureSensor>> {
    let hwmon = stream::select(hwmon(), hwmon_device());

    // We need the `thermal_zone` items, only if `hwmon` stream yielded nothing
    hwmon.choose_chain(thermal_zone())
}
