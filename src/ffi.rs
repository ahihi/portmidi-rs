use libc::{c_char, c_void};

pub type CPortMidiStream = c_void;

#[doc(hidden)]
pub type CPmMessage = i32;

pub type CPmTimestamp = u32;

#[doc(hidden)]
#[repr(C)]
pub struct CPmEvent {
    pub message : CPmMessage,
    pub timestamp : CPmTimestamp,
}

#[deriving(Show, FromPrimitive)]
#[repr(C)]
pub enum PmError {
    PmNoError = 0,
    PmGotData = 1, /* < A "no error" return that also indicates data available */
    PmHostError = -10000,
    PmInvalidDeviceId = -9999, /* out of range or
                                * output device when input is requested or
                                * input device when output is requested or
                                * device is already opened
                                */
    PmInsufficientMemory = -9998,
    PmBufferTooSmall = -9997,
    PmBufferOverflow = -9996,
    PmBadPtr = -9995, /* PortMidiStream parameter is NULL or
                       * stream is not opened or
                       * stream is output when input is required or
                       * stream is input when output is required */
    PmBadData = -9994, /* illegal midi data, e.g. missing EOX */
    PmInternalError = -9993,
    PmBufferMaxSize = -9992, /* buffer is already as large as it can be */
}

#[link(name = "portmidi")]
extern "C" {
    pub fn Pm_Initialize() -> PmError;
    pub fn Pm_Terminate()-> PmError;
    pub fn Pm_HasHostError(stream : *const CPortMidiStream ) -> i32;
    pub fn Pm_GetErrorText(errorCode : PmError) -> *const c_char;
    pub fn Pm_GetHostErrorText(msg : *const c_char , len : i32 );
    pub fn Pm_CountDevices() -> u32;
    pub fn Pm_GetDefaultInputDeviceID() -> super::CPmDeviceID;
    pub fn Pm_GetDefaultOutputDeviceID() -> super::CPmDeviceID;
    pub fn Pm_GetDeviceInfo(id:super::CPmDeviceID) -> *const super::CPmDeviceInfo;
    pub fn Pm_OpenInput(stream: *const *const CPortMidiStream, inputDevice : super::CPmDeviceID, inputDriverInfo: *const c_void, bufferSize : i32, time_proc: *const c_void, time_info: *const c_void) -> PmError;
    pub fn Pm_OpenOutput(stream : *const *const CPortMidiStream, outputDevice : super::CPmDeviceID, inputDriverInfo: *const c_void, bufferSize : i32, time_proc: *const c_void, time_info: *const c_void, latency:i32) -> PmError;
    pub fn Pm_Read(stream : *const CPortMidiStream, buffer : *mut CPmEvent , length : i32) -> i16;
    pub fn Pm_Abort(stream : *const CPortMidiStream) -> PmError;
    pub fn Pm_Close(stream : *const CPortMidiStream) -> PmError;
    pub fn Pm_Poll(stream : *const CPortMidiStream) -> PmError;
    pub fn Pm_Write(stream : *const CPortMidiStream, buffer : *const CPmEvent , length : i32) -> PmError;
    pub fn Pm_WriteShort(stream : *const CPortMidiStream, timestamp : CPmTimestamp , message : CPmMessage) -> PmError;
}
