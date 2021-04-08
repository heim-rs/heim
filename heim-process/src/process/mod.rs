use std::fmt;
use std::path::PathBuf;
use std::time::Instant;

use heim_common::prelude::*;
use heim_common::units::Time;

use crate::{sys, Pid, ProcessResult};

mod command;
mod cpu_times;
mod cpu_usage;
mod env;
mod io_counters;
mod memory;
mod status;

pub use self::command::{Command, CommandIter};
pub use self::cpu_times::CpuTime;
pub use self::cpu_usage::CpuUsage;
pub use self::env::{Environment, EnvironmentIter, IntoEnvironmentIter};
pub use self::io_counters::IoCounters;
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

    /// Returns process parent pid.
    pub async fn parent_pid(&self) -> ProcessResult<Pid> {
        self.as_ref().parent_pid().await
    }

    /// Returns parent [Process].
    ///
    /// [Process]: ./struct.Process.html
    pub async fn parent(&self) -> ProcessResult<Process> {
        let ppid = self.parent_pid().await?;

        get(ppid).await
    }

    /// Returns process name.
    pub async fn name(&self) -> ProcessResult<String> {
        self.as_ref().name().await
    }

    /// Returns process executable as an absolute path.
    pub async fn exe(&self) -> ProcessResult<PathBuf> {
        self.as_ref().exe().await
    }

    /// Returns process command line.
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
        self.as_ref().command().await.map(Into::into)
    }

    /// Returns process current working directory.
    ///
    /// ## Compatibility
    ///
    /// For Windows this method is implemented, but considered as an **unstable** right now.
    ///
    /// Please use it with caution and report any bugs you might face.
    pub async fn cwd(&self) -> ProcessResult<PathBuf> {
        self.as_ref().cwd().await
    }

    /// Returns current process status.
    pub async fn status(&self) -> ProcessResult<Status> {
        self.as_ref().status().await
    }

    /// Returns process environment.
    pub async fn environment(&self) -> ProcessResult<Environment> {
        self.as_ref().environment().await.map(Into::into)
    }

    /// Returns process creation time, expressed as a [Time] amount since the UNIX epoch.
    ///
    /// [Time]: ../units/type.Time.html
    pub async fn create_time(&self) -> ProcessResult<Time> {
        self.as_ref().create_time().await
    }

    /// Returns accumulated process time.
    pub async fn cpu_time(&self) -> ProcessResult<CpuTime> {
        self.as_ref().cpu_time().await.map(Into::into)
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
    pub async fn cpu_usage(&self) -> ProcessResult<CpuUsage> {
        let (cpu_time, cpu_count) = future::try_join(
            self.cpu_time(),
            heim_cpu::logical_count().map_err(Into::into),
        )
        .await?;

        Ok(CpuUsage {
            cpu_count,
            cpu_time,
            at: Instant::now(),
        })
    }

    /// Returns memory usage information for this process.
    pub async fn memory(&self) -> ProcessResult<Memory> {
        self.as_ref().memory().await.map(Into::into)
    }

    /// Checks if this `Process` is still running.
    pub async fn is_running(&self) -> ProcessResult<bool> {
        self.as_ref().is_running().await
    }

    /// Suspends the current process.
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
        self.as_ref().suspend().await
    }

    /// Resumes the current process.
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
        self.as_ref().resume().await
    }

    /// Terminates the current process.
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
        self.as_ref().terminate().await
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
        self.as_ref().kill().await
    }

    /// Wait for the current process termination.
    ///
    /// ## Returns
    ///
    /// If the process is already terminated, this method returns `Ok(())`.
    pub async fn wait(&self) -> ProcessResult<()> {
        self.as_ref().wait().await
    }

    /// Returns future which resolves into process IO counters.
    pub async fn io_counters(&self) -> ProcessResult<IoCounters> {
        self.as_ref().io_counters().await.map(Into::into)
    }
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Process").field("pid", &self.pid()).finish()
    }
}

/// Returns a stream over the currently running processes.
pub async fn processes() -> Result<impl Stream<Item = ProcessResult<Process>>> {
    let inner = sys::processes().await?;

    Ok(inner.map_ok(Into::into))
}

/// Loads the process information with `pid` given.
pub async fn get(pid: Pid) -> ProcessResult<Process> {
    sys::get(pid).await.map(Into::into)
}

/// Returns the `Process` matching the currently running program.
pub async fn current() -> ProcessResult<Process> {
    sys::current().await.map(Into::into)
}
