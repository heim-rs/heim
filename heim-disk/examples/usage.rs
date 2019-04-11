use heim_common::prelude::*;
use heim_disk as disk;
use heim_runtime::{self as runtime, SyncRuntime};

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        static USAGE_PATH: &'static str = "/";

    } else if #[cfg(windows)] {
        static USAGE_PATH: &'static str = "C:\\";
    } else {
        compile_error!("Unsupported OS for this example");
    }

}

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    let usage = rt.block_run(disk::usage(USAGE_PATH))?;
    println!("{:?}", usage);

    Ok(())
}
