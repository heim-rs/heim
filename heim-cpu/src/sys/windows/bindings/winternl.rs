#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::mem;
use std::ptr;

use winapi::shared::{minwindef, ntdef, ntstatus};

use heim_common::prelude::*;
use heim_common::sys::windows as ntdll;

use super::get_system_info;

#[repr(C)]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: ntdef::LARGE_INTEGER,
    pub KernelTime: ntdef::LARGE_INTEGER,
    pub UserTime: ntdef::LARGE_INTEGER,
    pub DpcTime: ntdef::LARGE_INTEGER,
    pub InterruptTime: ntdef::LARGE_INTEGER,
    pub InterruptCount: minwindef::ULONG,
}

#[repr(C)]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    IdleProcessTime: ntdef::LARGE_INTEGER,
    IoReadTransferCount: ntdef::LARGE_INTEGER,
    IoWriteTransferCount: ntdef::LARGE_INTEGER,
    IoOtherTransferCount: ntdef::LARGE_INTEGER,
    IoReadOperationCount: minwindef::ULONG,
    IoWriteOperationCount: minwindef::ULONG,
    IoOtherOperationCount: minwindef::ULONG,
    AvailablePages: minwindef::ULONG,
    CommittedPages: minwindef::ULONG,
    CommitLimit: minwindef::ULONG,
    PeakCommitment: minwindef::ULONG,
    PageFaultCount: minwindef::ULONG,
    CopyOnWriteCount: minwindef::ULONG,
    TransitionCount: minwindef::ULONG,
    CacheTransitionCount: minwindef::ULONG,
    DemandZeroCount: minwindef::ULONG,
    PageReadCount: minwindef::ULONG,
    PageReadIoCount: minwindef::ULONG,
    CacheReadCount: minwindef::ULONG,
    CacheIoCount: minwindef::ULONG,
    DirtyPagesWriteCount: minwindef::ULONG,
    DirtyWriteIoCount: minwindef::ULONG,
    MappedPagesWriteCount: minwindef::ULONG,
    MappedWriteIoCount: minwindef::ULONG,
    PagedPoolPages: minwindef::ULONG,
    NonPagedPoolPages: minwindef::ULONG,
    PagedPoolAllocs: minwindef::ULONG,
    PagedPoolFrees: minwindef::ULONG,
    NonPagedPoolAllocs: minwindef::ULONG,
    NonPagedPoolFrees: minwindef::ULONG,
    FreeSystemPtes: minwindef::ULONG,
    ResidentSystemCodePage: minwindef::ULONG,
    TotalSystemDriverPages: minwindef::ULONG,
    TotalSystemCodePages: minwindef::ULONG,
    NonPagedPoolLookasideHits: minwindef::ULONG,
    PagedPoolLookasideHits: minwindef::ULONG,
    AvailablePagedPoolPages: minwindef::ULONG,
    ResidentSystemCachePage: minwindef::ULONG,
    ResidentPagedPoolPage: minwindef::ULONG,
    ResidentSystemDriverPage: minwindef::ULONG,
    CcFastReadNoWait: minwindef::ULONG,
    CcFastReadWait: minwindef::ULONG,
    CcFastReadResourceMiss: minwindef::ULONG,
    CcFastReadNotPossible: minwindef::ULONG,
    CcFastMdlReadNoWait: minwindef::ULONG,
    CcFastMdlReadWait: minwindef::ULONG,
    CcFastMdlReadResourceMiss: minwindef::ULONG,
    CcFastMdlReadNotPossible: minwindef::ULONG,
    CcMapDataNoWait: minwindef::ULONG,
    CcMapDataWait: minwindef::ULONG,
    CcMapDataNoWaitMiss: minwindef::ULONG,
    CcMapDataWaitMiss: minwindef::ULONG,
    CcPinMappedDataCount: minwindef::ULONG,
    CcPinReadNoWait: minwindef::ULONG,
    CcPinReadWait: minwindef::ULONG,
    CcPinReadNoWaitMiss: minwindef::ULONG,
    CcPinReadWaitMiss: minwindef::ULONG,
    CcCopyReadNoWait: minwindef::ULONG,
    CcCopyReadWait: minwindef::ULONG,
    CcCopyReadNoWaitMiss: minwindef::ULONG,
    CcCopyReadWaitMiss: minwindef::ULONG,
    CcMdlReadNoWait: minwindef::ULONG,
    CcMdlReadWait: minwindef::ULONG,
    CcMdlReadNoWaitMiss: minwindef::ULONG,
    CcMdlReadWaitMiss: minwindef::ULONG,
    CcReadAheadIos: minwindef::ULONG,
    CcLazyWriteIos: minwindef::ULONG,
    CcLazyWritePages: minwindef::ULONG,
    CcDataFlushes: minwindef::ULONG,
    CcDataPages: minwindef::ULONG,
    pub ContextSwitches: minwindef::ULONG,
    FirstLevelTbFills: minwindef::ULONG,
    SecondLevelTbFills: minwindef::ULONG,
    pub SystemCalls: minwindef::ULONG,
    // Win10 declaration also has these fields.
    // Would it be okay just leave them or should there be
    // some dynamically chosen padding based on the Windows version?
    // See also: https://www.geoffchappell.com/studies/windows/km/ntoskrnl/api/ex/sysinfo/performance.htm
    //
    //ULONGLONG CcTotalDirtyPages;
    //ULONGLONG CcDirtyPageThreshold;
    //LONGLONG ResidentAvailablePages;
    //ULONGLONG SharedCommittedPages;
}

#[repr(C)]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub ContextSwitches: minwindef::ULONG,
    pub DpcCount: minwindef::ULONG,
    pub DpcRate: minwindef::ULONG,
    pub TimeIncrement: minwindef::ULONG,
    pub DpcBypassCount: minwindef::ULONG,
    pub ApcBypassCount: minwindef::ULONG,
}

pub trait SystemInformation: Sized {
    fn class() -> ntdll::SYSTEM_INFORMATION_CLASS;
}

impl SystemInformation for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn class() -> ntdll::SYSTEM_INFORMATION_CLASS {
        ntdll::SystemProcessorPerformanceInformation
    }
}

impl SystemInformation for SYSTEM_PERFORMANCE_INFORMATION {
    fn class() -> ntdll::SYSTEM_INFORMATION_CLASS {
        ntdll::SystemPerformanceInformation
    }
}

impl SystemInformation for SYSTEM_INTERRUPT_INFORMATION {
    fn class() -> ntdll::SYSTEM_INFORMATION_CLASS {
        ntdll::SystemInterruptInformation
    }
}

// Safe wrapper around the `NtQuerySystemInformation`
pub fn query_system_information<T>() -> Result<Vec<T>>
where
    T: SystemInformation,
{
    let info = unsafe { get_system_info() };
    let proc_amount = info.dwNumberOfProcessors as usize;
    if proc_amount == 0 {
        return Err(Error::incompatible("No processors were found"));
    }

    let mut info = Vec::<T>::with_capacity(proc_amount);
    let buffer_length = proc_amount * mem::size_of::<T>();

    unsafe {
        let result = ntdll::NtQuerySystemInformation(
            T::class(),
            info.as_mut_ptr() as *mut _,
            buffer_length as u32,
            ptr::null_mut(),
        )?;
        if result != ntstatus::STATUS_SUCCESS {
            return Err(Error::last_os_error());
        }
        info.set_len(proc_amount);
    };

    debug_assert!(!info.is_empty());

    Ok(info)
}
