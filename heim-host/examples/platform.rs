use heim_common::prelude::*;
use heim_host as host;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut runtime = runtime::new()?;
    let platform = runtime.block_run(host::platform())?;

    println!("{:?}", platform);

    Ok(())
}
