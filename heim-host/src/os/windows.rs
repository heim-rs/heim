use std::net::IpAddr;

#[heim_derive::os_ext_for(crate::User, cfg(target_os = "windows"))]
pub trait UserExt {
    fn domain(&self) -> &str;

    // TODO: Not all possible protocols are supported at the moment by the sys impl.
    // When they are will be implemented fully, this function should return `&IpAddr` directly,
    // without `Option<T>` wrapper.
    fn address(&self) -> Option<&IpAddr>;

}
