use epics_sys::{aiRecord, aoRecord};

use crate::record::{
    Scan, RecordType, FromRaw, Private, CommonPrivate,
    ScanHandler, ReadHandler, WriteHandler,
};


// Analog input

/// Handler trait for analog input
pub trait AiHandler: ScanHandler<AiRecord> + ReadHandler<AiRecord> {}

/// Analog input private data
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
derive_deref_record!(AiRecord);
unsafe impl Send for AiRecord {}


// Analog output

/// Handler trait for analog output
pub trait AoHandler: ScanHandler<AoRecord> + WriteHandler<AoRecord> {}

/// Analog output private data
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

/// Analog output record
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
derive_deref_record!(AoRecord);
unsafe impl Send for AoRecord {}
