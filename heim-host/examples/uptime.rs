#![allow(stable_features)]
#![feature(await_macro, async_await, futures_api)]

use heim_common::prelude::*;
use heim_host as host;

#[runtime::main]
async fn main() -> Result<()> {
    let uptime = await!(host::uptime())?;

    dbg!(uptime);

    Ok(())
}
