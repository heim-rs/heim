use heim_common::prelude::*;
use heim_host as host;

#[heim_derive::main]
async fn main() -> Result<()> {
    let uptime = host::uptime().await?;

    dbg!(uptime);

    Ok(())
}
