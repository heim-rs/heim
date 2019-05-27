use std::io;
use std::mem;
use std::ffi::CStr;

use winapi::um::{sysinfoapi, winnt, libloaderapi};
use winapi::shared::{ntdef, ntstatus, minwindef};

use heim_common::prelude::*;
use heim_common::sys::windows::get_ntdll;

use crate::Arch;

// Partial copy of the `sysinfoapi::SYSTEM_INFO`,
// because it contains pointers and we need to sent it between threads
struct SystemInfo {
    processor_arch: minwindef::WORD,
}

impl From<sysinfoapi::SYSTEM_INFO> for SystemInfo {
    fn from(info: sysinfoapi::SYSTEM_INFO) -> SystemInfo {
        let s = unsafe {
            info.u.s()
        };

        SystemInfo {
            processor_arch: s.wProcessorArchitecture,
        }
    }
}

pub struct Platform {
    sysinfo: SystemInfo,
    version: winnt::OSVERSIONINFOEXW,
    build: String,
}

impl Platform {
    pub fn system(&self) -> &str {
        match self.version.wProductType {
            winnt::VER_NT_WORKSTATION => "Windows",
            winnt::VER_NT_SERVER => "Windows Server",
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

    pub fn architecture(&self) -> Arch {
        match self.sysinfo.processor_arch {
            // While there are other `PROCESSOR_ARCHITECTURE_*` consts exists,
            // MSDN described only the following.
            // https://docs.microsoft.com/ru-ru/windows/desktop/api/sysinfoapi/ns-sysinfoapi-_system_info#members
            winnt::PROCESSOR_ARCHITECTURE_AMD64 => Arch::X86_64,
            winnt::PROCESSOR_ARCHITECTURE_ARM => Arch::ARM,
            winnt::PROCESSOR_ARCHITECTURE_ARM64 => Arch::AARCH64,
            // TODO: Is it okay to match Ia64 to unknown arch?
            // `platforms::Arch` enum does not have specific member to Itanium.
            winnt::PROCESSOR_ARCHITECTURE_IA64 => Arch::Unknown,
            winnt::PROCESSOR_ARCHITECTURE_INTEL => Arch::X86,
            _ => Arch::Unknown,
        }
    }
}

unsafe fn get_native_system_info() -> impl Future<Output=Result<SystemInfo>> {
    let mut info = mem::MaybeUninit::<sysinfoapi::SYSTEM_INFO>::uninit();
    sysinfoapi::GetNativeSystemInfo(info.as_mut_ptr());
    let info = info.assume_init();

    future::ok(info.into())
}

unsafe fn rtl_get_version() -> impl Future<Output=Result<winnt::OSVERSIONINFOEXW>> {
    // Based on the `platform-info` crate source:
    // https://github.com/uutils/platform-info/blob/8fa071f764d55bd8e41a96cf42009da9ae20a650/src/windows.rs
    let module = match get_ntdll() {
        Ok(module) => module,
        Err(e) => return future::err(e),
    };

    let funcname = CStr::from_bytes_with_nul_unchecked(b"RtlGetVersion\0");
    let func = libloaderapi::GetProcAddress(module, funcname.as_ptr());
    if !func.is_null() {
        let func: extern "stdcall" fn(*mut winnt::RTL_OSVERSIONINFOEXW)
            -> ntdef::NTSTATUS = mem::transmute(func as *const ());

        let mut osinfo = mem::MaybeUninit::<winnt::RTL_OSVERSIONINFOEXW>::uninit();
        (*osinfo.as_mut_ptr()).dwOSVersionInfoSize = mem::size_of::<winnt::RTL_OSVERSIONINFOEXW>() as minwindef::DWORD;
        if func(osinfo.as_mut_ptr()) == ntstatus::STATUS_SUCCESS {
            future::ok(osinfo.assume_init())
        } else {
            // https://docs.microsoft.com/en-us/windows/desktop/devnotes/rtlgetversion#return-value
            unreachable!("RtlGetVersion should just work");
        }
    } else {
        future::err(io::Error::last_os_error().into())
    }
}

pub fn platform() -> impl Future<Output=Result<Platform>> {
    let sysinfo = unsafe { get_native_system_info() };
    let version = unsafe { rtl_get_version() };

    future::join(sysinfo, version)
        .then(|result| {
            match result {
                (Ok(sysinfo), Ok(version)) => {
                    future::ok(Platform {
                        sysinfo,
                        version,
                        build: format!("{}", version.dwBuildNumber),
                    })
                },
                (Err(e), _) => future::err(e),
                (_, Err(e)) => future::err(e),
            }
        })
}
