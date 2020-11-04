//! Linux `uptime` implementation

use std::error::Error;
use std::time::Duration;

use futures::stream::TryStreamExt; // Needed for the `.try_fold` method
#[cfg(unix)]
use heim::units::ratio;
use heim::units::{time, Time};

fn format(t: Time) -> humantime::FormattedDuration {
    let duration = Duration::from_secs_f64(t.get::<time::second>());

    humantime::format_duration(duration)
}

fn main() -> Result<(), Box<dyn Error>> {
    smol::block_on(async {
        let uptime = heim::host::uptime().await?;
        // There is no `.count()` method for Streams
        let users = heim::host::users()
            .await?
            .try_fold(0usize, |acc, _| async move { Ok(acc + 1) })
            .await?;

        cfg_if::cfg_if! {
            if #[cfg(unix)] {
                // Load average is available for unixes only
                let (one, five, fifteen) = heim::cpu::os::unix::loadavg().await?;

                println!("up {}\t{} user,\tload average: {}, {}, {}",
                    format(uptime),
                    users,
                    one.get::<ratio::ratio>(),
                    five.get::<ratio::ratio>(),
                    fifteen.get::<ratio::ratio>(),
                );
            } else {
                println!("up {}\t{} user",
                    format(uptime),
                    users,
                );

            }
        }

        Ok(())
    })
}
