//use heim_common::prelude::*;
//use heim_net as net;
//use heim_runtime::{self as runtime, SyncRuntime};
//
//enum InetType {
//    Tcp4,
//    Tcp6,
//    Udp4,
//    Udp6,
//}
//
//fn print_inet(type_: InetType, conn: &net::InetConnection) {
//    let type_display = match type_ {
//        InetType::Tcp4 => "tcp4",
//        InetType::Tcp6 => "tcp6",
//        InetType::Udp4 => "udp4",
//        InetType::Udp6 => "udp6",
//    };
//    println!(
//        "{}\tstate={:?}\tsource={:?}\tdestination={:?}",
//        type_display,
//        conn.state(),
//        conn.source(),
//        conn.destination(),
//    );
//}
//
//fn print_unix(conn: &net::UnixConnection) {
//    println!("unix\tsource={:?}\tpeer={:?}", conn.source(), conn.peer(),);
//}
//
//fn main() -> Result<()> {
//    env_logger::init();
//    let mut rt = runtime::new().unwrap();
//
//    for conn in rt.block_collect(net::connections(net::ConnectionKind::all())).flatten() {
//        match &conn {
//            net::Connection::Tcp4(conn) => print_inet(InetType::Tcp4, conn),
//            net::Connection::Tcp6(conn) => print_inet(InetType::Tcp6, conn),
//            net::Connection::Udp4(conn) => print_inet(InetType::Udp4, conn),
//            net::Connection::Udp6(conn) => print_inet(InetType::Udp6, conn),
//            net::Connection::Unix(conn) => print_unix(conn),
//        };
//    }
//
//    Ok(())
//}

fn main() {}
