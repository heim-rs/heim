use heim_common::prelude::*;
use heim_host as host;

#[heim_derive::main]
async fn main() -> Result<()> {
    let boot_time = host::boot_time().await?;

    dbg!(boot_time);

    Ok(())
}
