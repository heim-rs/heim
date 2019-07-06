#![feature(async_await)]

use heim_common::prelude::*;
use heim_host as host;

#[runtime::main]
async fn main() -> Result<()> {
    let platform = host::platform().await?;

    dbg!(platform);

    Ok(())
}
