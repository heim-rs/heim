//! Windows-specific extensions.

use std::net::IpAddr;

/// Extension for [User] struct.
///
/// [User]: ../../struct.User.html
#[heim_derive::os_ext_for(crate::User, cfg(target_os = "windows"))]
pub trait UserExt {
    /// Domain name that the user belongs to.
    fn domain(&self) -> &str;

    // TODO: Not all possible protocols are supported at the moment by the sys impl.
    // When they are will be implemented fully, this function should return `&IpAddr` directly,
    // without `Option<T>` wrapper.
    // See https://github.com/heim-rs/heim/issues/63
    /// Client network address of a RDP session.
    ///
    /// At the moment not all possible protocols are supported
    /// (`AF_IPX`, `AF_NETBIOS` and `AF_UNSPEC` families are missing),
    /// and therefore, this method returns `Option<&IpAddr>`.
    ///
    /// It should be expected that method will return `&IpAddr` directly,
    /// when support for all protocols will arrive.
    fn address(&self) -> Option<&IpAddr>;
}
