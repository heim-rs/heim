use heim_common::prelude::*;
use heim_host as host;

#[heim_derive::main]
async fn main() -> Result<()> {
    let platform = host::platform().await?;

    dbg!(platform);

    Ok(())
}
