use std::ops::{Deref, DerefMut};

use libc::{c_int};

use epics_sys::{dbCommon, longinRecord, longoutRecord};

use super::{RecordBase, RecordRaw};

/// Integer input record
pub struct LonginRecord {
    raw: &'static mut longinRecord,
    base: RecordBase,
}

impl RecordRaw for LonginRecord {
    type Raw = *mut longinRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        let ptr = raw as *mut dbCommon;
        let base = RecordBase::from_raw(ptr);
        Self { raw: raw.as_mut().unwrap(), base }
    }
    unsafe fn init_raw(&mut self) {
        self.base.init_raw();
    }
}
impl LonginRecord {
    pub fn val(&self) -> i32 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: i32) {
        self.raw.val = val as c_int;
    }
}

impl Deref for LonginRecord {
    type Target = RecordBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for LonginRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// Integer output record
pub struct LongoutRecord {
    raw: &'static mut longoutRecord,
    base: RecordBase,
}

impl RecordRaw for LongoutRecord {
    type Raw = *mut longoutRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        let ptr = raw as *mut dbCommon;
        let base = RecordBase::from_raw(ptr);
        Self { raw: raw.as_mut().unwrap(), base }
    }
    unsafe fn init_raw(&mut self) {
        self.base.init_raw();
    }
}
impl LongoutRecord {
    pub fn val(&self) -> i32 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: i32) {
        self.raw.val = val as c_int;
    }
}

impl Deref for LongoutRecord {
    type Target = RecordBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for LongoutRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
