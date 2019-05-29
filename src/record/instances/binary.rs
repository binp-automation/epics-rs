use libc::{c_ushort};
use epics_sys::{biRecord, boRecord};

use crate::record::{
    Scan, RecordType, FromRaw, Private, CommonPrivate,
    ScanRecord, ReadRecord, WriteRecord,
    ScanHandler, ReadHandler, WriteHandler,
};


// Binary input

pub trait BiHandler: ScanHandler<BiRecord> + ReadHandler<BiRecord> {
    fn linconv(&mut self, record: &mut BiRecord, after: i32);
}

pub struct BiPrivate {
    base: CommonPrivate,
    handler: Option<Box<dyn BiHandler + Send>>,
}
impl BiPrivate {
    fn new(cpvt: CommonPrivate) -> Self {
        Self { base: cpvt, handler: None }
    }
}
derive_deref!(BiPrivate, CommonPrivate, base);
impl Private for BiPrivate {}


/// Binary input record
pub struct BiRecord {
    raw: &'static mut biRecord,
}
impl BiRecord {
    pub fn val(&self) -> bool {
        self.raw.val != 0
    }
    pub fn set_val(&mut self, val: bool) {
        self.raw.val = val as c_ushort;
    }
}
impl_record_private!(BiRecord, BiPrivate);
impl_record_handler!(BiRecord, BiHandler);

impl FromRaw for BiRecord {
    type Raw = *mut biRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}
derive_record!(BiRecord, BiPrivate);
derive_scan_record!(BiRecord);
derive_read_record!(BiRecord);

unsafe impl Send for BiRecord {}


// Binary output

// Handler for analog output record
pub trait BoHandler: ScanHandler<BoRecord> + WriteHandler<BoRecord> {
    fn linconv(&mut self, record: &mut BoRecord, after: i32);
}

pub struct BoPrivate {
    base: CommonPrivate,
    handler: Option<Box<dyn BoHandler + Send>>,
}
impl BoPrivate {
    fn new(cpvt: CommonPrivate) -> Self {
        Self { base: cpvt, handler: None }
    }
}
derive_deref!(BoPrivate, CommonPrivate, base);
impl Private for BoPrivate {}


/// Binary input record
pub struct BoRecord {
    raw: &'static mut boRecord,
}
impl BoRecord {
    pub fn val(&self) -> bool {
        self.raw.val != 0
    }
    pub fn set_val(&mut self, val: bool) {
        self.raw.val = val as c_ushort;
    }
}
impl_record_private!(BoRecord, BoPrivate);
impl_record_handler!(BoRecord, BoHandler);

impl FromRaw for BoRecord {
    type Raw = *mut boRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}
derive_record!(BoRecord, BoPrivate);
derive_scan_record!(BoRecord);
derive_write_record!(BoRecord);

unsafe impl Send for BoRecord {}
