use epics_sys::{aiRecord, aoRecord};

use crate::record::{
    Scan, RecordType, FromRaw, Private, CommonPrivate, Record,
    ScanHandler, ReadHandler, WriteHandler,
};


/// Record that can perform linconv (only AiRecord and AoRecord)
pub trait LinconvRecord: Record {
    unsafe fn handler_linconv(&mut self, after: i32);
}


// Analog input

pub trait AiHandler: ScanHandler<AiRecord> + ReadHandler<AiRecord> {
    fn linconv(&mut self, record: &mut AiRecord, after: i32);
}

pub struct AiPrivate {
    base: CommonPrivate,
    handler: Option<Box<dyn AiHandler + Send>>,
}
impl AiPrivate {
    fn new(cpvt: CommonPrivate) -> Self {
        Self { base: cpvt, handler: None }
    }
}
derive_deref!(AiPrivate, CommonPrivate, base);
impl Private for AiPrivate {}


/// Analog input record
pub struct AiRecord {
    raw: &'static mut aiRecord,
}
impl AiRecord {
    pub fn val(&self) -> f64 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: f64) {
        self.raw.val = val;
    }
}
impl_record_private!(AiRecord, AiPrivate);
impl_record_handler!(AiRecord, AiHandler);

impl FromRaw for AiRecord {
    type Raw = *mut aiRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}
derive_record!(AiRecord, AiPrivate);
derive_scan_record!(AiRecord);
derive_read_record!(AiRecord);
impl LinconvRecord for AiRecord {
    unsafe fn handler_linconv(&mut self, after: i32) {
        self.with_handler(|h, r| h.linconv(r, after))
    }
}
derive_deref_record!(AiRecord);
unsafe impl Send for AiRecord {}


// Analog output

// Handler for analog output record
pub trait AoHandler: ScanHandler<AoRecord> + WriteHandler<AoRecord> {
    fn linconv(&mut self, record: &mut AoRecord, after: i32);
}

pub struct AoPrivate {
    base: CommonPrivate,
    handler: Option<Box<dyn AoHandler + Send>>,
}
impl AoPrivate {
    fn new(cpvt: CommonPrivate) -> Self {
        Self { base: cpvt, handler: None }
    }
}
derive_deref!(AoPrivate, CommonPrivate, base);
impl Private for AoPrivate {}


/// Analog input record
pub struct AoRecord {
    raw: &'static mut aoRecord,
}
impl AoRecord {
    pub fn val(&self) -> f64 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: f64) {
        self.raw.val = val;
    }
}
impl_record_private!(AoRecord, AoPrivate);
impl_record_handler!(AoRecord, AoHandler);

impl FromRaw for AoRecord {
    type Raw = *mut aoRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}

derive_record!(AoRecord, AoPrivate);
derive_scan_record!(AoRecord);
derive_write_record!(AoRecord);
impl LinconvRecord for AoRecord {
    unsafe fn handler_linconv(&mut self, after: i32) {
        self.with_handler(|h, r| h.linconv(r, after))
    }
}
derive_deref_record!(AoRecord);
unsafe impl Send for AoRecord {}
