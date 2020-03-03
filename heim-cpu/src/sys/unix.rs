use heim_common::units::{ratio::ratio, Ratio};
use heim_common::{Error, Result};

pub async fn loadavg() -> Result<(Ratio, Ratio, Ratio)> {
    let mut data: [libc::c_double; 3] = [0.0, 0.0, 0.0];
    let result = unsafe { libc::getloadavg(data.as_mut_ptr(), 3) };

    if result == 3 {
        Ok((
            Ratio::new::<ratio>(data[0] as f32),
            Ratio::new::<ratio>(data[1] as f32),
            Ratio::new::<ratio>(data[2] as f32),
        ))
    } else {
        Err(Error::last_os_error().with_ffi("getloadavg"))
    }
}
