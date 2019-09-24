//! Sébastien Duquette (https://github.com/ekse) created the bindings for `wtsapi32`
//! and they are were intended to be merged into `winapi` crate here:
//! https://github.com/retep998/winapi-rs/pull/650/files
//!
//! For some reasons it was not, therefore, partially bundling them as is.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use winapi::shared::minwindef::{BOOL, BYTE, DWORD};
use winapi::um::winnt::{HANDLE, LARGE_INTEGER, LPWSTR, PSID, PVOID, WCHAR};

pub const WTS_CURRENT_SERVER_HANDLE: HANDLE = 0 as HANDLE;
const USERNAME_LENGTH: usize = 20;
const WINSTATIONNAME_LENGTH: usize = 32;
const DOMAIN_LENGTH: usize = 17;

ENUM! {enum WTS_CONNECTSTATE_CLASS {
    WTSActive,
    WTSConnected,
    WTSConnectQuery,
    WTSShadow,
    WTSDisconnected,
    WTSIdle,
    WTSListen,
    WTSReset,
    WTSDown,
    WTSInit,
}}

ENUM! {enum WTS_INFO_CLASS {
    WTSInitialProgram,
    WTSApplicationName,
    WTSWorkingDirectory,
    WTSOEMId,
    WTSSessionId,
    WTSUserName,
    WTSWinStationName,
    WTSDomainName,
    WTSConnectState,
    WTSClientBuildNumber,
    WTSClientName,
    WTSClientDirectory,
    WTSClientProductId,
    WTSClientHardwareId,
    WTSClientAddress,
    WTSClientDisplay,
    WTSClientProtocolType,
    WTSIdleTime,
    WTSLogonTime,
    WTSIncomingBytes,
    WTSOutgoingBytes,
    WTSIncomingFrames,
    WTSOutgoingFrames,
    WTSClientInfo,
    WTSSessionInfo,
    WTSSessionInfoEx,
    WTSConfigInfo,
    WTSValidationInfo,
    WTSSessionAddressV4,
    WTSIsRemoteSession,
}}

STRUCT! {struct WTS_SESSION_INFOW {
    SessionId: DWORD,
    pWinStationName: LPWSTR,
    State: WTS_CONNECTSTATE_CLASS,
}}
pub type PWTS_SESSION_INFOW = *mut WTS_SESSION_INFOW;

STRUCT! {struct WTS_PROCESS_INFOW {
    SessionId: DWORD,
    ProcessId: DWORD,
    pProcessName: LPWSTR,
    pUserSid: PSID,
}}
//pub type PWTS_PROCESS_INFOW = *mut WTS_PROCESS_INFOW;

STRUCT! {struct WTS_CLIENT_ADDRESS {
    AddressFamily: DWORD,
    Address: [BYTE; 20],
}}
pub type PWTS_CLIENT_ADDRESS = *mut WTS_CLIENT_ADDRESS;

STRUCT! {struct WTSINFOW {
    State: WTS_CONNECTSTATE_CLASS,
    SessionId: DWORD,
    IncomingBytes: DWORD,
    OutgoingBytes: DWORD,
    IncomingFrames: DWORD,
    OutgoingFrames: DWORD,
    IncomingCompressedBytes: DWORD,
    OutgoingCompressedBytes: DWORD,
    WinStationName: [WCHAR; WINSTATIONNAME_LENGTH],
    Domain: [WCHAR; DOMAIN_LENGTH],
    UserName: [WCHAR; USERNAME_LENGTH + 1],
    ConnectTime: LARGE_INTEGER,
    DisconnectTime: LARGE_INTEGER,
    LastInputTime: LARGE_INTEGER,
    LogonTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
}}
pub type PWTSINFOW = *mut WTSINFOW;

extern "system" {
    pub fn WTSEnumerateSessionsW(
        hServer: HANDLE,
        Reserved: DWORD,
        Version: DWORD,
        ppSessionInfo: *mut PWTS_SESSION_INFOW,
        pCount: *mut DWORD,
    ) -> BOOL;
    pub fn WTSQuerySessionInformationW(
        hServer: HANDLE,
        SessionId: DWORD,
        WTSInfoClass: WTS_INFO_CLASS,
        ppBuffer: *mut LPWSTR,
        pBytesReturned: *mut DWORD,
    ) -> BOOL;
    pub fn WTSFreeMemory(pMemory: PVOID);
}
