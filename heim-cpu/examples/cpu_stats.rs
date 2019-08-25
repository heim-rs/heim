use heim_common::prelude::*;
use heim_cpu as cpu;

#[heim_derive::main]
async fn main() -> Result<()> {
    dbg!(cpu::stats().await?);

    Ok(())
}
