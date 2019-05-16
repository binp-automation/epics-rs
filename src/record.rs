use std::ops::{Deref};

use std::ffi::{CStr};

use libc::{c_ushort};

use crate::epics::{dbCommon, aiRecord, aoRecord, biRecord, boRecord};

/// Common EPICS record
pub struct Record {
    raw: &'static mut dbCommon,
}

impl Record {
    pub(crate) fn new(raw: &'static mut dbCommon) -> Self {
        Self { raw }
    }

    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.raw.name.as_ptr()) }.to_str().unwrap()
    }
}

/// Analog input record
pub struct AiRecord {
    raw: &'static mut aiRecord,
    base: Record,
}

impl AiRecord {
    pub(crate) fn new(raw: &'static mut aiRecord) -> Self {
        let ptr = (raw as *mut aiRecord) as *mut dbCommon;
        let base = Record::new(unsafe{ &mut *ptr });
        Self { raw, base }
    }
    pub fn val(&self) -> f64 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: f64) {
        self.raw.val = val;
    }
}

impl Deref for AiRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Analog output record
pub struct AoRecord {
    raw: &'static mut aoRecord,
    base: Record,
}

impl AoRecord {
    pub(crate) fn new(raw: &'static mut aoRecord) -> Self {
        let ptr = (raw as *mut aoRecord) as *mut dbCommon;
        let base = Record::new(unsafe{ &mut *ptr });
        Self { raw, base }
    }
    pub fn val(&self) -> f64 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: f64) {
        self.raw.val = val;
    }
}

impl Deref for AoRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Binary input record
pub struct BiRecord {
    raw: &'static mut biRecord,
    base: Record,
}

impl BiRecord {
    pub(crate) fn new(raw: &'static mut biRecord) -> Self {
        let ptr = (raw as *mut biRecord) as *mut dbCommon;
        let base = Record::new(unsafe{ &mut *ptr });
        Self { raw, base }
    }
    pub fn val(&self) -> bool {
        self.raw.val != 0
    }
    pub fn set_val(&mut self, val: bool) {
        self.raw.val = val as c_ushort;
    }
}

impl Deref for BiRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Binary output record
pub struct BoRecord {
    raw: &'static mut boRecord,
    base: Record,
}

impl BoRecord {
    pub(crate) fn new(raw: &'static mut boRecord) -> Self {
        let ptr = (raw as *mut boRecord) as *mut dbCommon;
        let base = Record::new(unsafe{ &mut *ptr });
        Self { raw, base }
    }
    pub fn val(&self) -> bool {
        self.raw.val != 0
    }
    pub fn set_val(&mut self, val: bool) {
        self.raw.val = val as c_ushort;
    }
}

impl Deref for BoRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}