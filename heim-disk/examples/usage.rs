use heim_common::prelude::*;
use heim_disk as disk;

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        static USAGE_PATH: &str = "/";

    } else if #[cfg(windows)] {
        static USAGE_PATH: &str = "C:\\";
    } else {
        compile_error!("Unsupported OS for this example");
    }

}

#[heim_derive::main]
async fn main() -> Result<()> {
    let usage = disk::usage(USAGE_PATH).await?;
    dbg!(usage);

    Ok(())
}
