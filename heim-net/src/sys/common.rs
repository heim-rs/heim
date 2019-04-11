use heim_common::{Error, ErrorKind};

use crate::SocketType;

impl try_from::TryFrom<libc::c_int> for SocketType {
    type Err = Error;

    fn try_from(sock_type: libc::c_int) -> Result<Self, Self::Err> {
        match sock_type {
            libc::SOCK_STREAM => Ok(SocketType::Stream),
            libc::SOCK_DGRAM => Ok(SocketType::Datagram),
            libc::SOCK_SEQPACKET => Ok(SocketType::SeqPacket),
            libc::SOCK_DCCP => Ok(SocketType::Dccp),
            libc::SOCK_PACKET => Ok(SocketType::Packet),
            // TODO: Better error type
            _ => Err(Error::new(ErrorKind::Parse)),
        }
    }
}
