use heim_common::prelude::*;

use crate::{Connection, ConnectionKind, TcpState};

mod inet;
mod unix;

pub use self::inet::{inet_connections, InetConnection};
pub use self::unix::{unix_connections, UnixConnection};

type ConnectionStream = Box<dyn Stream<Item = Connection, Error = Error> + Send>;

// It seems that currently used `sock_diag` implementation
// improperly buffers outgoing requests,
// so we are doing it by ourselves, creating only one stream in a time.
//
// Note: seems like `sock_diag::Connection` will be closed when `self.handle` will be dropped.
// Hope we will not go out of max amount of available netlink sockets.
pub struct Connections {
    handle: sock_diag::Handle,
    kind: ConnectionKind,
    inner: Option<ConnectionStream>,
}

impl Connections {
    pub fn next_stream(&mut self) -> Option<ConnectionStream> {
        if self.kind.contains(ConnectionKind::TCP4) {
            trace!("Tcp4 connections was requested, invoking new stream");
            let f = inet_connections(&self.handle, libc::AF_INET, libc::IPPROTO_TCP);
            self.kind.set(ConnectionKind::TCP4, false);
            return Some(Box::new(f));
        }

        if self.kind.contains(ConnectionKind::TCP6) {
            trace!("Tcp6 connections was requested, invoking new stream");
            let f = inet_connections(&self.handle, libc::AF_INET6, libc::IPPROTO_TCP);
            self.kind.set(ConnectionKind::TCP6, false);
            return Some(Box::new(f));
        }

        if self.kind.contains(ConnectionKind::UDP4) {
            trace!("Udp4 connections was requested, invoking new stream");
            let f = inet_connections(&self.handle, libc::AF_INET, libc::IPPROTO_UDP);
            self.kind.set(ConnectionKind::UDP4, false);
            return Some(Box::new(f));
        }

        if self.kind.contains(ConnectionKind::UDP6) {
            trace!("Udp6 connections was requested, invoking new stream");
            let f = inet_connections(&self.handle, libc::AF_INET6, libc::IPPROTO_UDP);
            self.kind.set(ConnectionKind::UDP6, false);
            return Some(Box::new(f));
        }

        if self.kind.contains(ConnectionKind::UNIX) {
            trace!("Unix connections was requested, invoking new stream");
            let f = unix_connections(&self.handle);
            self.kind.set(ConnectionKind::UNIX, false);
            return Some(Box::new(f));
        }

        None
    }
}

impl Stream for Connections {
    type Item = Connection;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.inner.as_mut() {
                None => {
                    match self.next_stream() {
                        s @ Some(_) => {
                            self.inner = s;
                            continue;
                        }
                        // All streams had ended
                        None => return Ok(Async::Ready(None)),
                    }
                }
                Some(s) => match s.poll()? {
                    Async::Ready(None) => {
                        self.inner = None;
                        continue;
                    }
                    res => return Ok(res),
                },
            }
        }
    }
}

pub fn connections(kind: ConnectionKind) -> impl Stream<Item = Connection, Error = Error> {
    future::lazy(sock_diag::new_connection)
        .map_err(Error::from)
        .and_then(move |(conn, handle)| {
            tokio::executor::spawn(conn.map_err(|_| ()));

            let conns = Connections {
                handle,
                kind,
                inner: None,
            };

            Ok(conns)
        })
        .flatten_stream()
}

impl From<u8> for TcpState {
    fn from(value: u8) -> TcpState {
        match value {
            0 => TcpState::Unknown,
            1 => TcpState::Established,
            2 => TcpState::SynSent,
            3 => TcpState::SynRecv,
            4 => TcpState::FinWait1,
            5 => TcpState::FinWait2,
            6 => TcpState::TimeWait,
            7 => TcpState::Close,
            8 => TcpState::CloseWait,
            9 => TcpState::LastAck,
            10 => TcpState::Listen,
            11 => TcpState::Closing,
            other => unreachable!("Unknown TCP state {}", other),
        }
    }
}
