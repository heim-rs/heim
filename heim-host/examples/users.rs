#![allow(stable_features)]
#![feature(await_macro, async_await, futures_api)]

use heim_common::prelude::*;
use heim_host as host;

cfg_if::cfg_if! {
    if #[cfg(all(unix, not(target_os = "openbsd")))] {
        use heim_host::os::unix::UserExt;
    } else if #[cfg(target_os = "windows")] {
        use heim_host::os::windows::UserExt;
    }
}

#[runtime::main]
async fn main() -> Result<()> {
    let mut users = host::users();
    while let Some(user) = await!(users.next()) {
        let user = user?;

        println!("{:?}", user);

        println!("Extra:");

        #[cfg(all(unix, not(target_os = "openbsd")))]
        println!("Pid: {:?}", user.pid());
        #[cfg(all(unix, not(target_os = "openbsd")))]
        println!("Terminal: {:?}", user.terminal());

        #[cfg(target_os = "windows")]
        println!("Domain: {}", user.domain());
        #[cfg(target_os = "windows")]
        println!("Address: {:?}", user.address());
    }

    Ok(())
}
