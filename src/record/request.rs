use std::ptr;

use epics_sys::{
    IOSCANPVT, scanIoInit, scanIoRequest,
    CALLBACK, callbackRequest,
};


/// Scan handle for process requests from outside
#[derive(Debug, Clone)]
pub struct Scan {
    raw: IOSCANPVT,
}
impl Scan {
    pub(crate) fn new() -> Self {
        let mut scan = ptr::null_mut();
        unsafe { scanIoInit((&mut scan) as *mut _); }
        Self { raw: scan }
    }
    pub fn request(&self) -> Result<(),()> {
        match unsafe { scanIoRequest(self.raw) } {
            0 => Err(()),
            _ => Ok(()),
        }
    }
    pub(crate) unsafe fn as_raw(&self) -> &IOSCANPVT {
        &self.raw
    }
}
unsafe impl Send for Scan {}
unsafe impl Sync for Scan {}


/// Callback information for asynchronous processing
pub(crate) struct Callback {
    raw: CALLBACK,
}
impl Callback {
    pub(crate) fn new(raw: CALLBACK) -> Self {
        Self { raw }
    }
    pub(crate) fn request(&mut self) -> Result<(),()> {
        match unsafe { callbackRequest(&mut self.raw as *mut _) } {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}
unsafe impl Send for Callback {}
