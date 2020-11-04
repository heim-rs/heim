//! `uname -a` implementation
//!
use std::error::Error;

use heim::host;

fn main() -> Result<(), Box<dyn Error>> {
    smol::block_on(async {
        let platform = host::platform().await?;

        println!(
            "{} {} {} {} {}",
            platform.system(),
            platform.release(),
            platform.hostname(),
            platform.version(),
            platform.architecture().as_str(),
        );

        Ok(())
    })
}
