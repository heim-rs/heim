//! This module provides some routines related to the network stuff,
//! but used in other `heim-*` crates, therefore it is hidden.
//!
//! Please, do not use it directly, since there is no guarantees on API stability,
//! but use it via other crates.

use heim_common::prelude::*;
use heim_common::Pid;

use crate::{sys, IoCounters};

/// Returns stream which yield [IO counters] for each network interface for process with given `pid`.
///
/// **MUST** be used as `process::Process::io_counters()`
///
/// ## Compatibility
///
/// Implemented only for Linux for now. For other platforms will return an empty stream.
pub fn io_counters_for_pid(pid: Pid) -> impl Stream<Item = Result<IoCounters>> {
    sys::io_counters_for_pid(pid).map_ok(Into::into)
}
