use std::ops;
use std::time::Instant;

use heim_common::units::{ratio, time, Ratio};

use super::CpuTime;

/// Process CPU usage measurement.
///
/// See [Process::cpu_usage](./struct.Process.html#method.cpu_usage) method for details.
#[derive(Debug)]
pub struct CpuUsage {
    pub(crate) cpu_count: u64,
    pub(crate) cpu_time: CpuTime,
    pub(crate) at: Instant,
}

impl ops::Sub<CpuUsage> for CpuUsage {
    type Output = Ratio;

    #[allow(clippy::suspicious_arithmetic_impl, clippy::cast_lossless)]
    fn sub(self, rhs: CpuUsage) -> Self::Output {
        let delta_proc = (self.cpu_time.user() - rhs.cpu_time.user())
            + (self.cpu_time.system() - rhs.cpu_time.system());
        let delta_time = self.at - rhs.at;

        // TODO: Can be replaced with a `delta_time.as_secs_f64()`
        // as soon as https://github.com/rust-lang/rust/issues/54361 will be stable
        const NANOS_PER_SEC: u32 = 1_000_000_000;
        let mut delta_time_secs =
            (delta_time.as_secs() as f64) + (delta_time.as_nanos() as f64) / (NANOS_PER_SEC as f64);

        // Time should calculated across all the cores available
        delta_time_secs *= self.cpu_count as f64;

        if delta_time_secs != 0.0 {
            let overall_cpus_ratio = delta_proc.get::<time::second>() / delta_time_secs;
            let single_cpu_ratio = overall_cpus_ratio * self.cpu_count as f64;

            Ratio::new::<ratio::ratio>(single_cpu_ratio as f32)
        } else {
            Ratio::new::<ratio::ratio>(0.0)
        }
    }
}
