use heim_common::prelude::*;
use heim_net as net;

#[cfg(target_os = "linux")]
use heim_net::os::linux::IoCountersExt;

#[cfg(target_os = "windows")]
use heim_net::os::windows::IoCountersExt;

#[heim_derive::test]
async fn smoke_io_counters() {
    let mut counters = net::io_counters();
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
}

#[heim_derive::test]
async fn smoke_nic() {
    let mut nic = net::nic();
    while let Some(iface) = nic.next().await {
        let iface = iface.unwrap();

        let _ = iface.name();
        let _ = iface.address();
        let _ = iface.netmask();
        let _ = iface.destination();
        let _ = iface.is_up();
        let _ = iface.is_loopback();
        let _ = iface.is_multicast();

        #[cfg(target_os = "linux")]
        {
            use heim_net::os::linux::NicExt;

            let _ = iface.broadcast();
            let _ = iface.is_broadcast();
            let _ = iface.is_point_to_point();
        }

        #[cfg(target_os = "macos")]
        {
            use heim_net::os::macos::NicExt;

            let _ = iface.broadcast();
            let _ = iface.is_broadcast();
            let _ = iface.is_point_to_point();
        }
    }
}
