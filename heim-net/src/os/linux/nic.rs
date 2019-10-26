use crate::Address;

/// Linux-specific extension for [Nic].
///
/// [Nic]: ../../struct.Nic.html
pub trait NicExt {
    /// Returns broadcast address if available.
    fn broadcast(&self) -> Option<Address>;

    /// Returns `bool` indicating whether interface is broadcast.
    fn is_broadcast(&self) -> bool;

    /// Returns `bool` indicating whether interface is point-to-point.
    fn is_point_to_point(&self) -> bool;
}

#[cfg(target_os = "linux")]
impl NicExt for crate::Nic {
    fn broadcast(&self) -> Option<Address> {
        self.as_ref().broadcast()
    }

    fn is_broadcast(&self) -> bool {
        self.as_ref().is_broadcast()
    }

    fn is_point_to_point(&self) -> bool {
        self.as_ref().is_point_to_point()
    }
}
