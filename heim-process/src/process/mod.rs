use std::fmt;
use std::path::PathBuf;
use std::time::Instant;

use heim_common::prelude::*;
use heim_common::units::Time;

use crate::{sys, Pid, ProcessResult};

mod command;
mod cpu_times;
mod cpu_usage;
mod memory;
mod status;

pub use self::command::{Command, CommandIter};
pub use self::cpu_times::CpuTime;
pub use self::cpu_usage::CpuUsage;
pub use self::memory::Memory;
pub use self::status::Status;

/// System process.
///
/// Some extra methods can be found in the [OS extensions](./os/index.html)
#[derive(Eq, PartialEq, Hash, heim_derive::ImplWrap)]
pub struct Process(sys::Process);

impl Process {
    /// Returns the process pid.
    pub fn pid(&self) -> Pid {
        self.as_ref().pid()
    }

    /// Returns future which resolves into the process parent pid.
    pub async fn parent_pid(&self) -> ProcessResult<Pid> {
        self.as_ref().parent_pid().await
    }

    /// Returns future which resolves into the parent [Process].
    ///
    /// [Process]: ./struct.Process.html
    pub async fn parent(&self) -> ProcessResult<Process> {
        self.parent_pid().and_then(get).await
    }

    /// Returns future which resolves into the process name.
    pub async fn name(&self) -> ProcessResult<String> {
        self.as_ref().name().await
    }

    /// Returns future which resolves into the process executable as an absolute path.
    pub async fn exe(&self) -> ProcessResult<PathBuf> {
        self.as_ref().exe().await
    }

    /// Returns future which resolves into the process command line.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use heim_process::{self as process, Process, ProcessResult};
    /// #
    /// # #[heim_derive::main]
    /// # async fn main() -> ProcessResult<()> {
    /// let process = process::current().await?;
    /// let command = process.command().await?;
    /// println!("Command line arguments:");
    /// for arg in &command {
    ///     println!("{:?}", arg);
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn command(&self) -> ProcessResult<Command> {
        self.as_ref().command().map_ok(Into::into).await
    }

    /// Returns future which resolves into the process current working directory.
    ///
    /// ## Compatibility
    ///
    /// For Windows this method is not implemented yet and will always return an error,
    /// see [#105](https://github.com/heim-rs/heim/issues/105).
    pub async fn cwd(&self) -> ProcessResult<PathBuf> {
        self.as_ref().cwd().await
    }

    /// Returns future which resolves into the current process status.
    pub async fn status(&self) -> ProcessResult<Status> {
        self.as_ref().status().await
    }

    /// Returns future which resolves into the process creation time,
    /// expressed as a [Time] amount since the UNIX epoch.
    ///
    /// [Time]: ../units/type.Time.html
    pub async fn create_time(&self) -> ProcessResult<Time> {
        self.as_ref().create_time().await
    }

    /// Returns future which resolves into the accumulated process time.
    pub async fn cpu_time(&self) -> ProcessResult<CpuTime> {
        self.as_ref().cpu_time().map_ok(Into::into).await
    }

    /// Returns future which resolves into the CPU usage measurement.
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
    /// # use heim_process::{self as process, Process, ProcessResult};
    /// #
    /// # #[heim_derive::main]
    /// # async fn main() -> ProcessResult<()> {
    /// let process = process::current().await?;
    /// let measurement_1 = process.cpu_usage().await?;
    /// // Or any other async timer at your choice
    /// futures_timer::Delay::new(Duration::from_millis(100)).await?;
    /// let measurement_2 = process.cpu_usage().await?;
    ///
    /// println!("CPU usage: {} %", (measurement_2 - measurement_1).get::<ratio::percent>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`CpuUsage`]: ./struct.CpuUsage.html
    pub async fn cpu_usage(&self) -> ProcessResult<CpuUsage> {
        Ok(CpuUsage {
            cpu_count: heim_cpu::logical_count().await?,
            cpu_time: self.cpu_time().await?,
            at: Instant::now(),
        })
    }

    /// Returns future which resolves into the memory information about this process.
    pub async fn memory(&self) -> ProcessResult<Memory> {
        self.as_ref().memory().map_ok(Into::into).await
    }

    /// Returns future which checks if this `Process` is still running.
    pub async fn is_running(&self) -> ProcessResult<bool> {
        self.as_ref().is_running().await
    }

    /// Suspend the current process.
    ///
    /// Before the signal send, it checks whether process PID has been reused,
    /// and if it is a case, [`NoSuchProcess`] error will be returned.
    ///
    /// ## Compatibility
    ///
    /// For *nix systems it sends the `SIGSTOP` signal to process.
    ///
    /// [`NoSuchProcess`]: ./enum.ProcessError.html#variant.NoSuchProcess
    pub async fn suspend(&self) -> ProcessResult<()> {
        self.0.suspend().await
    }

    /// Resume the current process.
    ///
    /// Before the signal send, it checks whether process PID has been reused,
    /// and if it is a case, [`NoSuchProcess`] error will be returned.
    ///
    /// ## Compatibility
    ///
    /// For *nix systems it sends the `SIGCONT` signal to process.
    ///
    /// [`NoSuchProcess`]: ./enum.ProcessError.html#variant.NoSuchProcess
    pub async fn resume(&self) -> ProcessResult<()> {
        self.0.resume().await
    }

    /// Terminate the current process.
    ///
    /// Before the signal send, it checks whether process PID has been reused,
    /// and if it is a case, [`NoSuchProcess`] error will be returned.
    ///
    /// ## Compatibility
    ///
    /// For *nix systems it sends the `SIGTERM` signal to process.
    ///
    /// For Windows it is an alias for the [`Process::kill`]
    ///
    /// [`NoSuchProcess`]: ./enum.ProcessError.html#variant.NoSuchProcess
    /// [`Process::kill`]: #method.kill
    pub async fn terminate(&self) -> ProcessResult<()> {
        self.0.terminate().await
    }

    /// Kills the current process.
    ///
    /// Before the signal send, it checks whether process PID has been reused,
    /// and if it is a case, [`NoSuchProcess`] error will be returned.
    ///
    /// ## Compatibility
    ///
    /// For *nix systems it sends the `SIGKILL` signal to process.
    ///
    /// [`TerminateProcess`] function is used for Windows,
    /// it initiates the termination but does not awaits for completion.
    ///
    /// [`NoSuchProcess`]: ./enum.ProcessError.html#variant.NoSuchProcess
    /// [`TerminateProcess`]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess
    pub async fn kill(&self) -> ProcessResult<()> {
        self.0.kill().await
    }
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Process").field("pid", &self.pid()).finish()
    }
}

/// Returns stream which yields currently running processes.
pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    sys::processes().map_ok(Into::into)
}

/// Load the process information with `pid` given.
pub fn get(pid: Pid) -> impl Future<Output = ProcessResult<Process>> {
    sys::get(pid).map_ok(Into::into)
}

/// Returns the `Process` matching the currently running program.
pub fn current() -> impl Future<Output = ProcessResult<Process>> {
    sys::current().map_ok(Into::into)
}
