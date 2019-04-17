use heim_common::prelude::*;
use heim_host as host;
use heim_runtime::{self as runtime, SyncRuntime};

cfg_if::cfg_if! {
    if #[cfg(all(target_os = "unix", not(target_os = "openbsd")))] {
        use heim_host::os::unix::UserExt;
    } else if #[cfg(target_os = "windows")] {
        use heim_host::os::windows::UserExt;
    }
}

fn main() -> Result<()> {
    let mut runtime = runtime::new()?;
    let users = runtime.block_collect(host::users());

    for user in users {
        let user = user?;
        println!("{:?}", user);

        println!("Extra:");

        #[cfg(all(target_os = "unix", not(target_os = "openbsd")))]
        println!("Terminal: {:?}", user.terminal());

        #[cfg(target_os = "windows")]
        println!("Domain: {}", user.domain());
        #[cfg(target_os = "windows")]
        println!("Address: {:?}", user.address());
    }

    Ok(())
}
