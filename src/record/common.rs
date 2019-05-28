use std::ops::DerefMut;
use std::ptr;

use libc::{c_int, c_void};

use epics_sys::{
    dbCommon,
    IOSCANPVT, scanIoInit, scanIoRequest,
    CALLBACK, callbackSetProcess, callbackRequest,
};

use crate::util::{cstr_to_slice};


/// Runtime record type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordType {
    Ai,
    Ao,
    Bi,
    Bo,
    Longin,
    Longout,
    Stringin,
    Stringout,
}

pub trait ScanHandler<R: Record> {
    /// Set scan handle for `I/O Intr` records.
    fn set_scan(&mut self, rec: &mut R, scan: Scan);
}
pub trait ReadHandler<R: Record> {
    /// Synchronous read request. *Should not block.*
    ///
    /// Returns:
    /// + true is done,
    /// + false if need to be performed asynchronously
    fn read(&mut self, rec: &mut R) -> bool;
    /// Asynchronous read request, may block.
    ///
    /// This operation is performed in separate thread
    /// from thread pool and then notifies the EPICS.
    fn read_async(&mut self, rec: &mut R);
}
pub trait WriteHandler<R: Record> {
    /// Synchronous write request. *Should not block.*
    ///
    /// Returns:
    /// + true is done,
    /// + false if need to be performed asynchronously
    fn write(&mut self, rec: &mut R) -> bool;
    /// Asynchronous write request, may block.
    ///
    /// This operation is performed in separate thread
    /// from thread pool and then notifies the EPICS.
    fn write_async(&mut self, rec: &mut R);
}

#[derive(Debug, Clone)]
pub struct Scan {
    raw: IOSCANPVT,
}
impl Scan {
    unsafe fn new() -> Self {
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
    fn new(raw: CALLBACK) -> Self {
        Self { raw }
    }
    pub unsafe fn request(&mut self) -> Result<(),()> {
        match callbackRequest(&mut self.raw as *mut _) {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}
unsafe impl Send for Callback {}

pub trait Private: DerefMut<Target=CommonPrivate> + Send {}

pub struct CommonPrivate {
    rtype: RecordType,
    callback: Callback,
    scan: Option<Scan>,
}

pub trait FromRaw {
    type Raw;
    unsafe fn from_raw(raw: Self::Raw) -> Self;
}

pub(crate) unsafe fn raw_private_init<P: Private>(raw: &mut dbCommon, pvt: P) {
    assert!(raw.dpvt.is_null());
    raw.dpvt = Box::leak(Box::new(pvt)) as *mut _ as *mut c_void;
}
pub(crate) unsafe fn raw_private_create(raw: &mut dbCommon, rtype: RecordType) -> CommonPrivate {
    let mut cb: CALLBACK = CALLBACK::default();
    callbackSetProcess(
        (&mut cb) as *mut _,
        raw.prio as c_int,
        raw as *mut _ as *mut c_void,
    );
    CommonPrivate {
        rtype,
        callback: Callback::new(cb),
        scan: None,
    }
}

/// Behavior that is common for all records
pub trait Record {
    unsafe fn as_raw(&self) -> &dbCommon;
    unsafe fn as_raw_mut(&mut self) -> &mut dbCommon;

    unsafe fn init(&mut self);

    fn rtype(&self) -> RecordType {
        unsafe { self.private() }.rtype
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

    unsafe fn private(&self) -> &Private;
    unsafe fn private_mut(&mut self) -> &mut Private;

    unsafe fn process(&mut self) -> Result<(),()> {
        let pvt = self.private_mut();
        pvt.callback.request()
    }
}

pub trait ScanRecord: Record {
    unsafe fn create_scan(&self) -> Scan {
        Scan::new()
    }
    unsafe fn set_scan(&mut self, scan: Scan) {
        assert!(self.private_mut().scan.replace(scan).is_none());
    }
    unsafe fn get_scan(&self) -> Option<Scan> {
        self.private().scan.clone()
    }
    unsafe fn handler_set_scan(&mut self, scan: Scan);
}

pub trait ReadRecord: Record {
    unsafe fn handler_read(&mut self) -> bool;
    unsafe fn handler_read_async(&mut self);
}

pub trait WriteRecord: Record {
    unsafe fn handler_write(&mut self) -> bool;
    unsafe fn handler_write_async(&mut self);
}
