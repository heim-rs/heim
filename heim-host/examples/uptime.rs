#![feature(async_await)]

use heim_common::prelude::*;
use heim_host as host;

#[runtime::main]
async fn main() -> Result<()> {
    let uptime = host::uptime().await?;

    dbg!(uptime);

    Ok(())
}
