use heim_common::prelude::*;

use crate::{sys, CpuFrequency};

/// Returns a stream over the per-[CPU frequencies] information.
///
/// Order of the stream is constant.
///
/// [CPU frequencies]: ../../struct.CpuFrequency.html
pub fn frequencies() -> impl Stream<Item = Result<CpuFrequency>> {
    // TODO: Looks ugly, fix this thing.
    // Problem is that we want to doc this function
    // no matter for what target are we building documentation,
    // but `sys::frequencies` obviously available for Linux only.
    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            sys::frequencies().map_ok(Into::into)
        } else {
            stream::iter(vec![])
        }
    }
}
