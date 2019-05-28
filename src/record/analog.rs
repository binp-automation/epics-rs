use std::ops::{Deref, DerefMut};

use epics_sys::{dbCommon, aiRecord, aoRecord};

use super::{
    raw_private_init, raw_private_create,
    Scan, RecordType, FromRaw, Private, CommonPrivate,
    Record, ScanRecord, ReadRecord, WriteRecord,
    ScanHandler, ReadHandler, WriteHandler,
};


pub trait LinconvRecord: Record {
    unsafe fn handler_linconv(&mut self, after: i32);
}

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
impl Deref for AiPrivate {
    type Target = CommonPrivate;
    fn deref(&self) -> &CommonPrivate {
        &self.base
    }
}
impl DerefMut for AiPrivate {
    fn deref_mut(&mut self) -> &mut CommonPrivate {
        &mut self.base
    }
} 
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
    unsafe fn private_ai(&self) -> &AiPrivate {
        (self.raw.dpvt as *const AiPrivate).as_ref().unwrap()
    }
    unsafe fn private_ai_mut(&mut self) -> &mut AiPrivate {
        (self.raw.dpvt as *mut AiPrivate).as_mut().unwrap()
    }
    pub unsafe fn set_handler(&mut self, h: Box<dyn AiHandler + Send>) {
        assert!(self.private_ai_mut().handler.replace(h).is_none());
    }
    pub unsafe fn take_handler(&mut self) -> Box<dyn AiHandler + Send> {
        self.private_ai_mut().handler.take().unwrap()
    }
    pub unsafe fn with_handler<F, R>(&mut self, f: F) -> R
    where F: FnOnce(&mut dyn AiHandler, &mut Self) -> R {
        let mut h = self.take_handler();
        let ret = f(h.as_mut(), self);
        self.set_handler(h);
        ret
    }
}
impl FromRaw for AiRecord {
    type Raw = *mut aiRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}
impl Record for AiRecord {
    unsafe fn as_raw(&self) -> &dbCommon {
        (self.raw as *const _ as *const dbCommon).as_ref().unwrap()
    }
    unsafe fn as_raw_mut(&mut self) -> &mut dbCommon {
        (self.raw as *mut _ as *mut dbCommon).as_mut().unwrap()
    }

    unsafe fn init(&mut self) {
        let cpvt = raw_private_create(self.as_raw_mut(), RecordType::Ai);
        let pvt = AiPrivate::new(cpvt);
        raw_private_init::<AiPrivate>(self.as_raw_mut(), pvt);
    }

    unsafe fn private(&self) -> &Private {
        self.private_ai()
    }
    unsafe fn private_mut(&mut self) -> &mut Private {
        self.private_ai_mut()
    }
}
impl ScanRecord for AiRecord {
    unsafe fn handler_set_scan(&mut self, scan: Scan) {
        self.with_handler(|h, r| h.set_scan(r, scan))
    }
}
impl ReadRecord for AiRecord {
    unsafe fn handler_read(&mut self) -> bool {
        self.with_handler(|h, r| h.read(r))
    }
    unsafe fn handler_read_async(&mut self) {
        self.with_handler(|h, r| h.read_async(r))
    }
}
impl LinconvRecord for AiRecord {
    unsafe fn handler_linconv(&mut self, after: i32) {
        self.with_handler(|h, r| h.linconv(r, after))
    }
}
unsafe impl Send for AiRecord {}


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
impl Deref for AoPrivate {
    type Target = CommonPrivate;
    fn deref(&self) -> &CommonPrivate {
        &self.base
    }
}
impl DerefMut for AoPrivate {
    fn deref_mut(&mut self) -> &mut CommonPrivate {
        &mut self.base
    }
} 
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
    unsafe fn private_ao(&self) -> &AoPrivate {
        (self.raw.dpvt as *const AoPrivate).as_ref().unwrap()
    }
    unsafe fn private_ao_mut(&mut self) -> &mut AoPrivate {
        (self.raw.dpvt as *mut AoPrivate).as_mut().unwrap()
    }
    pub unsafe fn set_handler(&mut self, handler: Box<dyn AoHandler + Send>) {
        assert!(self.private_ao_mut().handler.replace(handler).is_none());
    }
    pub unsafe fn take_handler(&mut self) -> Box<dyn AoHandler + Send> {
        self.private_ao_mut().handler.take().unwrap()
    }
    pub unsafe fn with_handler<F, R>(&mut self, f: F) -> R
    where F: FnOnce(&mut dyn AoHandler, &mut Self) -> R {
        let mut h = self.take_handler();
        let ret = f(h.as_mut(), self);
        self.set_handler(h);
        ret
    }
}
impl FromRaw for AoRecord {
    type Raw = *mut aoRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
}
impl Record for AoRecord {
    unsafe fn as_raw(&self) -> &dbCommon {
        (self.raw as *const _ as *const dbCommon).as_ref().unwrap()
    }
    unsafe fn as_raw_mut(&mut self) -> &mut dbCommon {
        (self.raw as *mut _ as *mut dbCommon).as_mut().unwrap()
    }

    unsafe fn init(&mut self) {
        let cpvt = raw_private_create(self.as_raw_mut(), RecordType::Ai);
        let pvt = AoPrivate::new(cpvt);
        raw_private_init::<AoPrivate>(self.as_raw_mut(), pvt);
    }

    unsafe fn private(&self) -> &Private {
        self.private_ao()
    }
    unsafe fn private_mut(&mut self) -> &mut Private {
        self.private_ao_mut()
    }
}
impl ScanRecord for AoRecord {
    unsafe fn handler_set_scan(&mut self, scan: Scan) {
        self.with_handler(|h, r| h.set_scan(r, scan))
    }
}
impl WriteRecord for AoRecord {
    unsafe fn handler_write(&mut self) -> bool {
        self.with_handler(|h, r| h.write(r))
    }
    unsafe fn handler_write_async(&mut self) {
        self.with_handler(|h, r| h.write_async(r))
    }
}
impl LinconvRecord for AoRecord {
    unsafe fn handler_linconv(&mut self, after: i32) {
        self.with_handler(|h, r| h.linconv(r, after))
    }
}
unsafe impl Send for AoRecord {}
