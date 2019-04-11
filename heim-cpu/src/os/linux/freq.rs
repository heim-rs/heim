use heim_common::prelude::*;

use crate::{sys, CpuFrequency};

/// Returns stream of per-[CPU frequencies] information.
///
/// Order of the stream is constant.
///
/// [CPU frequencies]: struct.CpuFrequency.html
pub fn frequencies() -> impl Stream<Item=CpuFrequency, Error=Error> {
    sys::frequencies().map(Into::into)
}
