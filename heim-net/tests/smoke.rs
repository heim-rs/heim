use heim_common::prelude::*;
use heim_net as net;

#[cfg(target_os = "linux")]
use heim_net::os::linux::IoCountersExt;

#[cfg(target_os = "windows")]
use heim_net::os::windows::IoCountersExt;

#[heim_derive::test]
async fn smoke_io_counters() -> Result<()> {
    let counters = net::io_counters().await?;
    ::futures::pin_mut!(counters);
    while let Some(counter) = counters.next().await {
        let counter = counter.unwrap();

        let _ = counter.interface();
        let _ = counter.bytes_sent();
        let _ = counter.bytes_recv();
        let _ = counter.packets_sent();
        let _ = counter.packets_recv();
        let _ = counter.errors_sent();
        let _ = counter.errors_recv();
        let _ = counter.drop_recv();

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let _ = counter.drop_sent();
    }

    Ok(())
}

#[heim_derive::test]
async fn smoke_nic() -> Result<()> {
    let nic = net::nic().await?;
    ::futures::pin_mut!(nic);
    while let Some(iface) = nic.next().await {
        let iface = iface?;

        println!("Interface");
        println!("  name: {:?}", iface.name());
        println!("  index: {:?}", iface.index());
        println!("  address: {:?}", iface.address());
        println!("  netmask: {:?}", iface.netmask());

        println!("  destination: {:?}", iface.destination());
        println!("  is_up: {:?}", iface.is_up());
        println!("  is_running: {:?}", iface.is_running());
        println!("  is_loopback: {:?}", iface.is_loopback());
        println!("  is_multicast: {:?}", iface.is_multicast());

        #[cfg(target_os = "windows")]
        {
            use heim_net::os::windows::NicExt;

            println!("  guid: {:?}", iface.guid());
        }

        #[cfg(target_os = "linux")]
        {
            use heim_net::os::linux::NicExt;

            println!("  broadcast: {:?}", iface.broadcast());
            println!("  is_broadcast: {:?}", iface.is_broadcast());
            println!("  is_point_to_point: {:?}", iface.is_point_to_point());
        }

        #[cfg(target_os = "macos")]
        {
            use heim_net::os::macos::NicExt;

            println!("  broadcast: {:?}", iface.broadcast());
            println!("  is_broadcast: {:?}", iface.is_broadcast());
            println!("  is_point_to_point: {:?}", iface.is_point_to_point());
        }
    }

    Ok(())
}
