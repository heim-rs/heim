pub use super::{InetConnection, UnixConnection};

pub type Inode = u64;

bitflags::bitflags! {
    /// Bitflags used for requesting different connection kinds.
    ///
    /// Used by [connections](fn.connections.html) function.
    /// See its documentation for more.
    pub struct ConnectionKind: u32 {
        const TCP4 = 0b00000001;
        const TCP6 = 0b00000010;
        const UDP4 = 0b00000100;
        const UDP6 = 0b00001000;
        const UNIX = 0b00010000;
    }
}

/// Possible TCP connection states.
///
/// **TODO**: Should not be called `TcpState`, since used for all `InetConnection`.
///
/// See [RFC 793](https://www.ietf.org/rfc/rfc793.txt).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TcpState {
    /// Is not defined by RFC, but might be returned by OS implementation
    /// when it is not possible to determine connection state.
    Unknown,
    /// Represents an open connection, data received can be delivered to the user.
    ///
    /// The normal state for the data transfer phase of the connection.
    Established,
    /// Represents waiting for a matching connection request after having sent a connection request.
    SynSent,
    /// Represents waiting for a confirming connection request acknowledgment
    /// after having both received and sent a connection request.
    SynRecv,
    /// Represents waiting for a connection termination request from the remote TCP,
    /// or an acknowledgment of the connection termination request previously sent.
    FinWait1,
    /// Represents waiting for a connection termination request from the remote TCP.
    FinWait2,
    /// Represents waiting for enough time to pass to be sure the remote TCP received
    /// the acknowledgment of its connection termination request.
    TimeWait,
    /// Represents no connection state at all.
    Close,
    /// Represents waiting for a connection termination request from the local user.
    CloseWait,
    /// Represents waiting for an acknowledgment of the connection termination request
    /// previously sent to the remote TCP. (which includes an acknowledgment
    /// of its connection termination request).
    LastAck,
    /// Represents waiting for a connection request from any remote TCP and port.
    Listen,
    /// Represents waiting for a connection termination request acknowledgment from the remote TCP.
    Closing,
}

#[doc(hidden)] // Not used yet
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SocketType {
    /// SOCK_STREAM
    Stream,
    /// SOCK_DGRAM
    Datagram,
    /// SOCK_SEQPACKET
    SeqPacket,
    /// SOCK_DCCP
    Dccp,
    /// SOCK_PACKET
    Packet,
    #[doc(hidden)]
    __Nonexhaustive,
}
