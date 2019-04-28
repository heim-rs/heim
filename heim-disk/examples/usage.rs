#![feature(await_macro, async_await, futures_api)]

use heim_common::prelude::*;
use heim_disk as disk;

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        static USAGE_PATH: &'static str = "/";

    } else if #[cfg(windows)] {
        static USAGE_PATH: &'static str = "C:\\";
    } else {
        compile_error!("Unsupported OS for this example");
    }

}

#[runtime::main]
async fn main() -> Result<()> {
    let usage = await!(disk::usage(USAGE_PATH));
    dbg!(usage);

    Ok(())
}
