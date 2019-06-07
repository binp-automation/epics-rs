use libc::{c_ushort};
use epics_sys::{biRecord, boRecord};

use crate::record::{
    Scan, RecordType, FromRaw, Private, CommonPrivate,
    ScanHandler, ReadHandler, WriteHandler,
};


// Binary input

/// Handler trait for binary input
pub trait BiHandler: ScanHandler<BiRecord> + ReadHandler<BiRecord> {
    impl_into_boxed_handler!(BiHandler);
}

/// Binary input private data
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
derive_stype!(BiRecord, Bi);
derive_record!(BiRecord, BiPrivate);
derive_scan_record!(BiRecord);
derive_read_record!(BiRecord);
derive_deref_record!(BiRecord);
unsafe impl Send for BiRecord {}


// Binary output

/// Handler trait for binary output
pub trait BoHandler: ScanHandler<BoRecord> + WriteHandler<BoRecord> {
    impl_into_boxed_handler!(BoHandler);
}

/// Binary output private data
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

/// Binary output record
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
derive_stype!(BoRecord, Bo);
derive_record!(BoRecord, BoPrivate);
derive_scan_record!(BoRecord);
derive_write_record!(BoRecord);
derive_deref_record!(BoRecord);
unsafe impl Send for BoRecord {}
