use heim_net as net;
use heim_runtime::{self as runtime, SyncRuntime};

#[test]
fn smoke_counters() {
    let mut rt = runtime::new().unwrap();
    let counters = rt.block_collect(net::io_counters());

    assert_ne!(0, counters.count());
}

//#[test]
//fn smoke_connections() {
//    let mut rt = runtime::new().unwrap();
//    let conns = rt.block_collect(net::connections(net::ConnectionKind::all()));
//
//    assert_ne!(0, conns.count());
//}

#[test]
fn smoke_nic() {
    let mut rt = runtime::new().unwrap();
    let counters = rt.block_collect(net::nic());

    assert_ne!(0, counters.count());
}
