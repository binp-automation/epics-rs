use std::ops::{DerefMut};
use std::ptr;

use libc::{c_int, c_void};

use epics_sys::{
    dbCommon,
    IOSCANPVT, scanIoInit, scanIoRequest,
    CALLBACK, callbackSetProcess, callbackRequest,
};

use crate::util::{cstr_to_slice};


#[derive(Debug, Clone)]
pub struct Scan {
    raw: IOSCANPVT,
}
impl Scan {
    pub unsafe fn new() -> Self {
        let mut scan = ptr::null_mut();
        scanIoInit((&mut scan) as *mut _);
        Self { raw: scan }
    }
    pub fn request(&self) -> Result<(),()> {
        match unsafe { scanIoRequest(self.raw) } {
            0 => Err(()),
            _ => Ok(()),
        }
    }
    pub unsafe fn as_raw(&self) -> &IOSCANPVT {
        &self.raw
    }
}
unsafe impl Send for Scan {}
unsafe impl Sync for Scan {}

pub struct Callback {
    raw: CALLBACK,
}

impl Callback {
    pub unsafe fn new(raw: CALLBACK) -> Self {
        Self { raw }
    }
    pub fn request(&mut self) -> Result<(),()> {
        match unsafe { callbackRequest(&mut self.raw as *mut _) } {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}

pub struct Private {
    callback: Option<Callback>,
    scan: Option<Scan>,
}
impl Private {
    pub fn new() -> Self {
        Self { callback: None, scan: None }
    }
}


/// Behavior that is common for all records
pub trait Record {
    type Private: DerefMut<Target=Private>;

    unsafe fn as_raw(&self) -> &dbCommon;
    unsafe fn as_raw_mut(&mut self) -> &mut dbCommon;

    unsafe fn init(&mut self) {
        self.as_raw_mut().dpvt = ptr::null_mut();
    }

    fn name(&self) -> &[u8] {
        cstr_to_slice(&unsafe { self.as_raw() }.name)
    }

    fn pact(&self) -> bool {
        unsafe { self.as_raw() }.pact != 0
    }
    unsafe fn set_pact(&mut self, pact: bool) {
        self.as_raw_mut().pact = if pact { 1 } else { 0 };
    }

    unsafe fn set_private(&mut self, private: Self::Private) {
        let mut raw = self.as_raw_mut();
        assert!(raw.dpvt.is_null());
        raw.dpvt = Box::leak(Box::new(private)) as *mut _ as *mut c_void;
    }
    unsafe fn private(&self) -> &Self::Private {
        (self.as_raw().dpvt as *const Self::Private).as_ref().unwrap()
    }
    unsafe fn private_mut(&mut self) -> &mut Self::Private {
        (self.as_raw_mut().dpvt as *mut Self::Private).as_mut().unwrap()
    }

    unsafe fn create_callback(&mut self) -> Callback {
        let mut cb: CALLBACK = CALLBACK::default();
        callbackSetProcess(
            (&mut cb) as *mut _,
            self.as_raw_mut().prio as c_int,
            self.as_raw_mut() as *mut _ as *mut c_void,
        );
        Callback::new(cb)
    }
    unsafe fn prepare(&mut self) {
        if self.private_mut().callback.is_none() {
            self.private_mut().callback = Some(self.create_callback());
        }
    }
    unsafe fn process(&mut self) -> Result<(),()> {
        let pvt = self.private_mut();
        pvt.callback.as_mut().unwrap().request()
    }

    fn get_scan(&self) -> Option<Scan> {
        unsafe { self.private() }.scan.clone()
    }
}
