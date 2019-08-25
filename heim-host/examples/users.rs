use heim_common::prelude::*;
use heim_host as host;

#[heim_derive::main]
async fn main() -> Result<()> {
    let mut users = host::users();
    while let Some(user) = users.next().await {
        let user = user?;

        println!("{:?}", user);
    }

    Ok(())
}
