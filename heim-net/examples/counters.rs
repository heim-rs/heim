use heim_common::prelude::*;
use heim_net as net;

#[heim_derive::main]
async fn main() -> Result2<()> {
    let counters = net::io_counters();
    pin_utils::pin_mut!(counters);
    while let Some(counter) = counters.next().await {
        dbg!(counter?);
    }

    Ok(())
}
