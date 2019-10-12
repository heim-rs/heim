use heim_common::prelude::*;
use heim_cpu as cpu;

#[heim_derive::main]
async fn main() -> Result2<()> {
    dbg!(cpu::time().await?);

    let times = cpu::times();
    pin_utils::pin_mut!(times);
    while let Some(time) = times.next().await {
        dbg!(time?);
    }

    Ok(())
}
