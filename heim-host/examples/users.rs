use heim_common::prelude::*;
use heim_host as host;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut runtime = runtime::new()?;
    let users = runtime.block_collect(host::users());

    for user in users {
        dbg!(user);
    }

    Ok(())
}
