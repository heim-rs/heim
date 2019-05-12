#![allow(stable_features)]
#![feature(async_await, futures_api)]

use heim_common::prelude::*;
use heim_host as host;

#[runtime::main]
async fn main() -> Result<()> {
    let platform = host::platform().await?;

    dbg!(platform);

    Ok(())
}
