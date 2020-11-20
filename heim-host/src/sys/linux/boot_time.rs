use heim_common::{
    units::{time, Time},
    Error, Result,
};
use heim_runtime as rt;

pub async fn boot_time() -> Result<Time> {
    let contents = rt::fs::read_to_string(rt::linux::procfs_root().join("stat")).await?;

    for line in contents.lines() {
        if line.starts_with("btime ") {
            let mut parts = line.splitn(2, ' ');
            let _ = parts.next();

            return match parts.next() {
                Some(raw_value) => raw_value
                    .parse::<f64>()
                    .map(Time::new::<time::second>)
                    .map_err(Into::into),
                None => Err(Error::missing_key(
                    "btime",
                    format!("{}/stat", rt::linux::procfs_root().display()),
                )),
            };
        }
    }

    Err(Error::missing_key(
        "btime",
        format!("{}/stat", rt::linux::procfs_root().display()),
    ))
}
