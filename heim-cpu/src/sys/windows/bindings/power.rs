use winapi::shared::minwindef;

#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct PROCESSOR_POWER_INFORMATION {
    pub Number: minwindef::ULONG,
    pub MaxMhz: minwindef::ULONG,
    pub CurrentMhz: minwindef::ULONG,
    pub MhzLimit: minwindef::ULONG,
    pub MaxIdleState: minwindef::ULONG,
    pub CurrentIdleState: minwindef::ULONG,
}
