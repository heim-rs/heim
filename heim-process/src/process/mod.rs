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
#[derive(Eq, PartialEq, Hash)]
pub struct Process(sys::Process);

wrap!(Process, sys::Process);

impl Process {
    /// Returns the process pid.
    pub fn pid(&self) -> Pid {
        self.as_ref().pid()
    }

    /// Returns future which resolves into the process parent pid.
    pub fn parent_pid(&self) -> impl Future<Output = ProcessResult<Pid>> {
        self.as_ref().parent_pid()
    }

    /// Returns future which resolves into the parent [Process].
    ///
    /// [Process]: ./struct.Process.html
    pub fn parent(&self) -> impl Future<Output = ProcessResult<Process>> {
        self.parent_pid().and_then(get)
    }

    /// Returns future which resolves into the process name.
    pub fn name(&self) -> impl Future<Output = ProcessResult<String>> {
        self.as_ref().name()
    }

    /// Returns future which resolves into the process executable as an absolute path.
    pub fn exe(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        self.as_ref().exe()
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
    pub fn command(&self) -> impl Future<Output = ProcessResult<Command>> {
        self.as_ref().command().map_ok(Into::into)
    }

    /// Returns future which resolves into the process current working directory.
    ///
    /// ## Compatibility
    ///
    /// For Windows this method is not implemented yet and will always return an error,
    /// see [#105](https://github.com/heim-rs/heim/issues/105).
    pub fn cwd(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        self.as_ref().cwd()
    }

    /// Returns future which resolves into the current process status.
    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        self.as_ref().status()
    }

    /// Returns future which resolves into the process creation time,
    /// expressed as a [Time] amount since the UNIX epoch.
    ///
    /// [Time]: ../units/type.Time.html
    pub fn create_time(&self) -> impl Future<Output = ProcessResult<Time>> {
        self.as_ref().create_time()
    }

    /// Returns future which resolves into the accumulated process time.
    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        self.as_ref().cpu_time().map_ok(Into::into)
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
    /// futures_timer::Delay::new(Duration::from_millis(100)).await;
    /// let measurement_2 = process.cpu_usage().await?;
    ///
    /// println!("CPU usage: {} %", (measurement_2 - measurement_1).get::<ratio::percent>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`CpuUsage`]: ./struct.CpuUsage.html
    pub fn cpu_usage(&self) -> impl Future<Output = ProcessResult<CpuUsage>> {
        self.cpu_time().and_then(|time| {
            heim_cpu::logical_count()
                .map_err(Into::into)
                .map_ok(move |count| CpuUsage {
                    cpu_count: count,
                    cpu_time: time,
                    at: Instant::now(),
                })
        })
    }

    /// Returns future which resolves into the memory information about this process.
    pub fn memory(&self) -> impl Future<Output = ProcessResult<Memory>> {
        self.as_ref().memory().map_ok(Into::into)
    }

    /// Returns future which checks if this `Process` is still running.
    pub fn is_running(&self) -> impl Future<Output = ProcessResult<bool>> {
        self.as_ref().is_running()
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
    pub fn suspend(&self) -> impl Future<Output = ProcessResult<()>> {
        self.0.suspend()
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
    pub fn resume(&self) -> impl Future<Output = ProcessResult<()>> {
        self.0.resume()
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
    pub fn terminate(&self) -> impl Future<Output = ProcessResult<()>> {
        self.0.terminate()
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
    pub fn kill(&self) -> impl Future<Output = ProcessResult<()>> {
        self.0.kill()
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
