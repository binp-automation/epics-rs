use epics_sys::{stringinRecord, stringoutRecord};
use crate::util::{cstr_to_slice, cstr_copy_from_slice};

use crate::record::{
    Scan, RecordType, FromRaw, Private, CommonPrivate,
    ScanRecord, ReadRecord, WriteRecord,
    ScanHandler, ReadHandler, WriteHandler,
};


// Analog input

pub trait StringinHandler: ScanHandler<StringinRecord> + ReadHandler<StringinRecord> {
    fn linconv(&mut self, record: &mut StringinRecord, after: i32);
}

pub struct StringinPrivate {
    base: CommonPrivate,
    handler: Option<Box<dyn StringinHandler + Send>>,
}
impl StringinPrivate {
    fn new(cpvt: CommonPrivate) -> Self {
        Self { base: cpvt, handler: None }
    }
}
derive_deref!(StringinPrivate, CommonPrivate, base);
impl Private for StringinPrivate {}


/// Analog input record
pub struct StringinRecord {
    raw: &'static mut stringinRecord,
}
impl StringinRecord {
    pub fn val(&self) -> &[u8] {
        cstr_to_slice(&self.raw.val)
    }
    pub fn set_val(&mut self, val: &[u8]) {
        cstr_copy_from_slice(&mut self.raw.val, val);
    }
}
impl_record_private!(StringinRecord, StringinPrivate);
impl_record_handler!(StringinRecord, StringinHandler);

impl FromRaw for StringinRecord {
    type Raw = *mut stringinRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}
derive_record!(StringinRecord, StringinPrivate);
derive_scan_record!(StringinRecord);
derive_read_record!(StringinRecord);

unsafe impl Send for StringinRecord {}


// Analog output

// Handler for analog output record
pub trait StringoutHandler: ScanHandler<StringoutRecord> + WriteHandler<StringoutRecord> {
    fn linconv(&mut self, record: &mut StringoutRecord, after: i32);
}

pub struct StringoutPrivate {
    base: CommonPrivate,
    handler: Option<Box<dyn StringoutHandler + Send>>,
}
impl StringoutPrivate {
    fn new(cpvt: CommonPrivate) -> Self {
        Self { base: cpvt, handler: None }
    }
}
derive_deref!(StringoutPrivate, CommonPrivate, base);
impl Private for StringoutPrivate {}


/// Analog input record
pub struct StringoutRecord {
    raw: &'static mut stringoutRecord,
}
impl StringoutRecord {
    pub fn val(&self) -> &[u8] {
        cstr_to_slice(&self.raw.val)
    }
    pub fn set_val(&mut self, val: &[u8]) {
        cstr_copy_from_slice(&mut self.raw.val, val);
    }
}
impl_record_private!(StringoutRecord, StringoutPrivate);
impl_record_handler!(StringoutRecord, StringoutHandler);

impl FromRaw for StringoutRecord {
    type Raw = *mut stringoutRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}

derive_record!(StringoutRecord, StringoutPrivate);
derive_scan_record!(StringoutRecord);
derive_write_record!(StringoutRecord);

unsafe impl Send for StringoutRecord {}
