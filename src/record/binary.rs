use std::ops::{Deref, DerefMut};

use libc::{c_ushort};

use epics_sys::{dbCommon, biRecord, boRecord};

use super::{RecordBase, RecordRaw};

/// Binary input record
pub struct BiRecord {
    raw: &'static mut biRecord,
    base: RecordBase,
}

impl RecordRaw for BiRecord {
    type Raw = *mut biRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        let ptr = raw as *mut dbCommon;
        let base = RecordBase::from_raw(ptr);
        Self { raw: raw.as_mut().unwrap(), base }
    }
    unsafe fn init_raw(&mut self) {
        self.base.init_raw();
    }
}
impl BiRecord {
    pub fn val(&self) -> bool {
        self.raw.val != 0
    }
    pub fn set_val(&mut self, val: bool) {
        self.raw.val = val as c_ushort;
    }
}

impl Deref for BiRecord {
    type Target = RecordBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for BiRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// Binary output record
pub struct BoRecord {
    raw: &'static mut boRecord,
    base: RecordBase,
}

impl RecordRaw for BoRecord {
    type Raw = *mut boRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        let ptr = raw as *mut dbCommon;
        let base = RecordBase::from_raw(ptr);
        Self { raw: raw.as_mut().unwrap(), base }
    }
    unsafe fn init_raw(&mut self) {
        self.base.init_raw();
    }
}
impl BoRecord {
    pub fn val(&self) -> bool {
        self.raw.val != 0
    }
    pub fn set_val(&mut self, val: bool) {
        self.raw.val = val as c_ushort;
    }
}

impl Deref for BoRecord {
    type Target = RecordBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for BoRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
