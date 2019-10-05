use heim_common::prelude::*;
use heim_cpu as cpu;

#[cfg(target_os = "linux")]
async fn linux_frequencies() -> Result2<()> {
    let frequencies = cpu::os::linux::frequencies();
    pin_utils::pin_mut!(frequencies);
    while let Some(freq) = frequencies.next().await {
        dbg!(freq?);
    }

    Ok(())
}

#[heim_derive::main]
async fn main() -> Result2<()> {
    let freq = cpu::frequency().await;
    dbg!(freq?);

    #[cfg(target_os = "linux")]
    linux_frequencies().await?;

    Ok(())
}
