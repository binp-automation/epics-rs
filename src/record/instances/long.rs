use libc::{c_int};
use epics_sys::{longinRecord, longoutRecord};

use crate::record::{
    Scan, RecordType, FromRaw, Private, CommonPrivate,
    ScanRecord, ReadRecord, WriteRecord,
    ScanHandler, ReadHandler, WriteHandler,
};


// Analog input

pub trait LonginHandler: ScanHandler<LonginRecord> + ReadHandler<LonginRecord> {
    fn linconv(&mut self, record: &mut LonginRecord, after: i32);
}

pub struct LonginPrivate {
    base: CommonPrivate,
    handler: Option<Box<dyn LonginHandler + Send>>,
}
impl LonginPrivate {
    fn new(cpvt: CommonPrivate) -> Self {
        Self { base: cpvt, handler: None }
    }
}
derive_deref!(LonginPrivate, CommonPrivate, base);
impl Private for LonginPrivate {}


/// Analog input record
pub struct LonginRecord {
    raw: &'static mut longinRecord,
}
impl LonginRecord {
    pub fn val(&self) -> i32 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: i32) {
        self.raw.val = val as c_int;
    }
}
impl_record_private!(LonginRecord, LonginPrivate);
impl_record_handler!(LonginRecord, LonginHandler);

impl FromRaw for LonginRecord {
    type Raw = *mut longinRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}
derive_record!(LonginRecord, LonginPrivate);
derive_scan_record!(LonginRecord);
derive_read_record!(LonginRecord);

unsafe impl Send for LonginRecord {}


// Analog output

// Handler for analog output record
pub trait LongoutHandler: ScanHandler<LongoutRecord> + WriteHandler<LongoutRecord> {
    fn linconv(&mut self, record: &mut LongoutRecord, after: i32);
}

pub struct LongoutPrivate {
    base: CommonPrivate,
    handler: Option<Box<dyn LongoutHandler + Send>>,
}
impl LongoutPrivate {
    fn new(cpvt: CommonPrivate) -> Self {
        Self { base: cpvt, handler: None }
    }
}
derive_deref!(LongoutPrivate, CommonPrivate, base);
impl Private for LongoutPrivate {}


/// Analog input record
pub struct LongoutRecord {
    raw: &'static mut longoutRecord,
}
impl LongoutRecord {
    pub fn val(&self) -> i32 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: i32) {
        self.raw.val = val as c_int;
    }
}
impl_record_private!(LongoutRecord, LongoutPrivate);
impl_record_handler!(LongoutRecord, LongoutHandler);

impl FromRaw for LongoutRecord {
    type Raw = *mut longoutRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}
derive_record!(LongoutRecord, LongoutPrivate);
derive_scan_record!(LongoutRecord);
derive_write_record!(LongoutRecord);

unsafe impl Send for LongoutRecord {}
