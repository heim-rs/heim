use heim_common::{
    units::{time, Time},
    Error, Result,
};
use heim_runtime as rt;

pub async fn uptime() -> Result<Time> {
    let contents = rt::fs::read_to_string(rt::linux::procfs_root().join("uptime")).await?;

    match contents.splitn(2, ' ').next() {
        Some(raw_value) => {
            let seconds = raw_value.parse::<f64>()?;

            Ok(Time::new::<time::second>(seconds))
        }
        None => Err(Error::missing_key(
            "uptime",
            format!("{}/uptime", rt::linux::procfs_root().display()),
        )),
    }
}
