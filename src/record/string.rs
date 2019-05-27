use crate::util::{cstr_to_slice, cstr_copy_from_slice};
use std::ops::{Deref, DerefMut};

use epics_sys::{dbCommon, stringinRecord, stringoutRecord};

use super::{RecordBase, RecordRaw};


/// String input record
pub struct StringinRecord {
    raw: &'static mut stringinRecord,
    base: RecordBase,
}

impl RecordRaw for StringinRecord {
    type Raw = *mut stringinRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        let ptr = raw as *mut dbCommon;
        let base = RecordBase::from_raw(ptr);
        Self { raw: raw.as_mut().unwrap(), base }
    }
    unsafe fn init_raw(&mut self) {
        self.base.init_raw();
    }
}
impl StringinRecord {
    pub fn val(&self) -> &[u8] {
        cstr_to_slice(&self.raw.val)
    }
    pub fn set_val(&mut self, val: &[u8]) {
        cstr_copy_from_slice(&mut self.raw.val, val);
    }
}

impl Deref for StringinRecord {
    type Target = RecordBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for StringinRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// String output record
pub struct StringoutRecord {
    raw: &'static mut stringoutRecord,
    base: RecordBase,
}

impl RecordRaw for StringoutRecord {
    type Raw = *mut stringoutRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        let ptr = raw as *mut dbCommon;
        let base = RecordBase::from_raw(ptr);
        Self { raw: raw.as_mut().unwrap(), base }
    }
    unsafe fn init_raw(&mut self) {
        self.base.init_raw();
    }
}
impl StringoutRecord {
    pub fn val(&self) -> &[u8] {
        cstr_to_slice(&self.raw.val)
    }
    pub fn set_val(&mut self, val: &[u8]) {
        cstr_copy_from_slice(&mut self.raw.val, val);
    }
}

impl Deref for StringoutRecord {
    type Target = RecordBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for StringoutRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
