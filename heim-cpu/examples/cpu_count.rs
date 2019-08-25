use heim_common::prelude::*;
use heim_cpu as cpu;

#[heim_derive::main]
async fn main() -> Result<()> {
    dbg!(cpu::logical_count().await?);
    dbg!(cpu::physical_count().await?);

    Ok(())
}
