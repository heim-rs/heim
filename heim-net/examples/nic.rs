use heim_common::prelude::*;
use heim_net as net;

#[heim_derive::main]
async fn main() -> Result<()> {
    let nic = net::nic();
    pin_utils::pin_mut!(nic);
    while let Some(iface) = nic.next().await {
        dbg!(iface?);
    }

    Ok(())
}
