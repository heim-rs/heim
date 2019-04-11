//! Information system of quantities

system! {
    /// Information system of quantities, based on IEC 8000.
    quantities: IEC {
        information: bit, B;
    }
    /// Information System of units.
    units: U {
        information::Information,
    }
}

pub mod quantities {
    IEC!(crate::units::iec);
}

storage_types! {
    pub types: u64, usize;

    IEC!(crate::units::iec, V);
}
