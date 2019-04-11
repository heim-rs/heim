use heim_common::prelude::*;

use crate::sys;

mod inet;
mod types;
#[cfg(unix)]
mod unix;

pub use self::types::*;

pub use self::inet::InetConnection;
#[cfg(unix)]
pub use self::unix::UnixConnection;

#[derive(Debug)]
pub enum Connection {
    Tcp4(InetConnection),
    Tcp6(InetConnection),
    Udp4(InetConnection),
    Udp6(InetConnection),
    #[cfg(unix)]
    Unix(UnixConnection),
}

/// Returns stream which yields network [connections].
///
/// ## Examples
///
/// **TODO**: Broken at the moment
/// ```no_run,edition2018
/// # use heim_common::prelude::*;
/// # use heim_net::{ConnectionKind, connections};
/// # use heim_runtime as runtime;
/// # fn main() -> Result<()> {
/// let f = future::lazy(|| {
/// // Get all TCP4 & TCP6 connections
///         Ok(connections(ConnectionKind::TCP4 | ConnectionKind::TCP6))
///     })
///     .flatten_stream()
///     .map_err(|_| ())
///     .for_each(|conn| {
///         println!("{:?}", conn);
///         Ok(())
///     });
/// tokio::run(f);
/// # Ok(())
/// # }
/// ```
///
/// [connections]: enum.Connection.html
pub fn connections(kind: ConnectionKind) -> impl Stream<Item = Connection, Error = Error> {
    sys::connections(kind)
}
