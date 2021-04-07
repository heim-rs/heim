use std::fmt;
use std::mem;

use ntapi::ntrtl;
use winapi::shared::ntdef::NULL;
use winapi::shared::{minwindef, ntstatus};
use winapi::um::sysinfoapi::{
    ComputerNameDnsDomain, ComputerNameDnsHostname, COMPUTER_NAME_FORMAT,
};
use winapi::um::{sysinfoapi, winnt};

use heim_common::prelude::{Error, Result};

use crate::Arch;

// Partial copy of the `sysinfoapi::SYSTEM_INFO`,
// because it contains pointers and we need to sent it between threads.
// TODO: It would be better to make `SYSTEM_INFO` Sendable somehow?
#[derive(Debug)]
struct SystemInfo {
    processor_arch: minwindef::WORD,
}

impl From<sysinfoapi::SYSTEM_INFO> for SystemInfo {
    fn from(info: sysinfoapi::SYSTEM_INFO) -> SystemInfo {
        let s = unsafe { info.u.s() };

        SystemInfo {
            processor_arch: s.wProcessorArchitecture,
        }
    }
}

pub struct Platform {
    sysinfo: SystemInfo,
    version: winnt::OSVERSIONINFOEXW,
    hostname: String,
    domain: String,
    build: String,
}

impl Platform {
    pub fn system(&self) -> &str {
        match self.version.wProductType {
            winnt::VER_NT_WORKSTATION => "Windows",
            winnt::VER_NT_SERVER => "Windows Server",
            winnt::VER_NT_DOMAIN_CONTROLLER => "Windows Domain Controller",
            other => unreachable!("Unknown Windows product type: {}", other),
        }
    }

    pub fn release(&self) -> &str {
        // https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/content/wdm/ns-wdm-_osversioninfoexw#remarks

        let major = self.version.dwMajorVersion;
        let minor = self.version.dwMinorVersion;
        let suite_mask = minwindef::DWORD::from(self.version.wSuiteMask);
        let is_workstation = self.version.wProductType == winnt::VER_NT_WORKSTATION;

        match (major, minor) {
            (10, 0) => "10",
            (6, 3) => "8.1",
            (6, 2) if is_workstation => "8",
            (6, 2) if !is_workstation => "2012",
            (6, 1) if is_workstation => "7",
            (6, 1) if !is_workstation => "2008 R2",
            (6, 0) if is_workstation => "Vista",
            (6, 0) if !is_workstation => "2008",
            (5, 2) if suite_mask == winnt::VER_SUITE_WH_SERVER => "Home Server",
            (5, 2) if is_workstation => "XP Professional x64 Edition",
            (5, 2) => "2003",
            (5, 1) => "XP",
            (5, 0) => "2000",
            _ => "unknown",
        }
    }

    pub fn version(&self) -> &str {
        self.build.as_str()
    }

    pub fn hostname(&self) -> &str {
        self.hostname.as_str()
    }

    pub fn domain(&self) -> &str {
        self.domain.as_str()
    }

    pub fn architecture(&self) -> Arch {
        match self.sysinfo.processor_arch {
            // While there are other `PROCESSOR_ARCHITECTURE_*` consts exists,
            // MSDN described only the following.
            // https://docs.microsoft.com/ru-ru/windows/desktop/api/sysinfoapi/ns-sysinfoapi-_system_info#members
            winnt::PROCESSOR_ARCHITECTURE_AMD64 => Arch::X86_64,
            winnt::PROCESSOR_ARCHITECTURE_ARM => Arch::ARM,
            winnt::PROCESSOR_ARCHITECTURE_ARM64 => Arch::AARCH64,
            // TODO: Is it okay to match Ia64 to unknown arch?
            // `platforms::Arch` enum does not have specific member for Itanium.
            winnt::PROCESSOR_ARCHITECTURE_IA64 => Arch::Unknown,
            winnt::PROCESSOR_ARCHITECTURE_INTEL => Arch::X86,
            _ => Arch::Unknown,
        }
    }
}

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Platform")
            .field("system", &self.system())
            .field("release", &self.release())
            .field("version", &self.version())
            .field("hostname", &self.hostname())
            .field("architecture", &self.architecture())
            .finish()
    }
}

fn get_native_system_info() -> SystemInfo {
    let mut info = mem::MaybeUninit::<sysinfoapi::SYSTEM_INFO>::uninit();
    unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getnativesysteminfo
        // Returns nothing and can't fail, apparently
        sysinfoapi::GetNativeSystemInfo(info.as_mut_ptr());
        info.assume_init().into()
    }
}

/// Based on the `platform-info` crate source:
/// https://github.com/uutils/platform-info/blob/8fa071f764d55bd8e41a96cf42009da9ae20a650/src/windows.rs
fn rtl_get_version() -> winnt::OSVERSIONINFOEXW {
    let mut osinfo = mem::MaybeUninit::<winnt::RTL_OSVERSIONINFOEXW>::uninit();

    unsafe {
        (*osinfo.as_mut_ptr()).dwOSVersionInfoSize =
            mem::size_of::<winnt::RTL_OSVERSIONINFOEXW>() as minwindef::DWORD;

        let result = ntrtl::RtlGetVersion(osinfo.as_mut_ptr() as *mut _);

        // Should work all the time.
        // https://docs.microsoft.com/en-us/windows/desktop/devnotes/rtlgetversion#return-value
        debug_assert!(result == ntstatus::STATUS_SUCCESS);

        osinfo.assume_init()
    }
}

fn get_value_from_get_computer_name_ex_w(kind: COMPUTER_NAME_FORMAT) -> Result<String> {
    let mut required_size: minwindef::DWORD = 0;
    let result = unsafe { sysinfoapi::GetComputerNameExW(kind, NULL as _, &mut required_size) };
    if result != 0 {
        return Err(Error::last_os_error().with_ffi("GetComputerNameEx"));
    }
    // required_size does not contain the trailing null byte
    let mut size = required_size + 1;

    let mut buffer: Vec<winnt::WCHAR> = vec![0; size as _]; // this ensures that buffer.len = size already (avoiding the need to resort to the unsafe `set_len` later)
    let result = unsafe { sysinfoapi::GetComputerNameExW(kind, buffer.as_mut_ptr(), &mut size) };
    if result == 0 {
        return Err(Error::last_os_error().with_ffi("GetComputerNameEx"));
    }

    if size > required_size {
        // Should not happen, size "receives the number of TCHARs copied to the destination buffer, not including the terminating null character"
        let e = std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid value returned by GetComputerNameExW",
        );
        return Err(e.into());
    }
    // buffer[..size] is valid because buffer.len > size already
    let str = String::from_utf16_lossy(&buffer[..(size as usize)]);
    Ok(str)
}

fn get_computer_name() -> Result<String> {
    get_value_from_get_computer_name_ex_w(ComputerNameDnsHostname)
}

fn get_computer_domain() -> Result<String> {
    get_value_from_get_computer_name_ex_w(ComputerNameDnsDomain)
}

pub async fn platform() -> Result<Platform> {
    let version = rtl_get_version();

    Ok(Platform {
        sysinfo: get_native_system_info(),
        version,
        hostname: get_computer_name()?,
        domain: get_computer_domain()?,
        build: format!("{}", version.dwBuildNumber),
    })
}
