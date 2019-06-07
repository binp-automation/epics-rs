use epics_sys::{stringinRecord, stringoutRecord};
use crate::util::{cstr_array_read_bytes, cstr_array_write_bytes};

use crate::record::{
    Scan, RecordType, FromRaw, Private, CommonPrivate,
    ScanHandler, ReadHandler, WriteHandler,
};


// String input

/// Handler trait for string input
pub trait StringinHandler: ScanHandler<StringinRecord> + ReadHandler<StringinRecord> {
    impl_into_boxed_handler!(StringinHandler);
}

/// String input private data
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

/// String input record
pub struct StringinRecord {
    raw: &'static mut stringinRecord,
}
impl StringinRecord {
    pub fn val(&self) -> &[u8] {
        cstr_array_read_bytes(&self.raw.val)
    }
    pub fn set_val(&mut self, val: &[u8]) {
        cstr_array_write_bytes(&mut self.raw.val, val);
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
derive_stype!(StringinRecord, Stringin);
derive_record!(StringinRecord, StringinPrivate);
derive_scan_record!(StringinRecord);
derive_read_record!(StringinRecord);
derive_deref_record!(StringinRecord);
unsafe impl Send for StringinRecord {}


// String output

/// Handler trait for string output
pub trait StringoutHandler: ScanHandler<StringoutRecord> + WriteHandler<StringoutRecord> {
    impl_into_boxed_handler!(StringoutHandler);
}

/// String output private data
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

/// String output record
pub struct StringoutRecord {
    raw: &'static mut stringoutRecord,
}
impl StringoutRecord {
    pub fn val(&self) -> &[u8] {
        cstr_array_read_bytes(&self.raw.val)
    }
    pub fn set_val(&mut self, val: &[u8]) {
        cstr_array_write_bytes(&mut self.raw.val, val);
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
derive_stype!(StringoutRecord, Stringout);
derive_record!(StringoutRecord, StringoutPrivate);
derive_scan_record!(StringoutRecord);
derive_write_record!(StringoutRecord);
derive_deref_record!(StringoutRecord);
unsafe impl Send for StringoutRecord {}
