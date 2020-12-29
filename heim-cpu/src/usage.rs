use std::ops;
use std::time::Instant;

use heim_common::prelude::*;
use heim_common::units::{ratio, time, Ratio};

use super::{logical_count, time, CpuTime};

/// System CPU usage measurement.
///
/// See [cpu_usage](./fn.cpu_usage.html) method for details.
#[derive(Debug, Clone)]
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

        let mut delta_time_secs = delta_time.as_secs_f64();

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

/// Returns CPU usage measurement.
///
/// Returned [`CpuUsage`] struct represents instantaneous CPU usage and does not represent
/// any reasonable value by itself.
/// It is suggested to wait for a while with help of any async timer
/// (for accuracy recommended delay should be at least 100 ms),
/// call this method once again and subtract former [`CpuUsage`] from the new one.
///
/// Same to any *nix system, calculated CPU usage might exceed 100 %
/// if the process is running multiple threads on different CPU cores.
///
/// ## Example
///
/// ```rust
/// # use std::time::Duration;
/// # use heim_common::units::ratio;
/// # use heim_common::prelude::*;
/// # use heim_cpu::usage;
/// #
/// # #[heim_derive::main]
/// # async fn main() -> Result<()> {
/// let measurement_1 = usage().await?;
/// // Or any other async timer at your choice
/// futures_timer::Delay::new(Duration::from_millis(100)).await;
/// let measurement_2 = usage().await?;
///
/// println!("CPU usage: {} %", (measurement_2 - measurement_1).get::<ratio::percent>());
/// # Ok(())
/// # }
/// ```
///
/// [`CpuUsage`]: ./struct.CpuUsage.html
pub async fn usage() -> Result<CpuUsage> {
    let (cpu_time, cpu_count) =
        future::try_join(time(), logical_count().map_err(Into::into)).await?;

    Ok(CpuUsage {
        cpu_count,
        cpu_time,
        at: Instant::now(),
    })
}
