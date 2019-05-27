use std::ops::{Deref, DerefMut};

use epics_sys::{dbCommon, aiRecord, aoRecord};

use super::{
    Record, Private, AnyRecordRef,
    AiHandler, AoHandler,
};

pub struct AiPrivate {
    base: Private,
    handler: Box<dyn AiHandler + Send>,
}

impl Deref for AiPrivate {
    type Target = Private;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for AiPrivate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// Analog input record
pub trait AiRecord: Record<Private=AiPrivate> {
    unsafe fn as_raw_ai(&self) -> &aiRecord;
    unsafe fn as_raw_ai_mut(&mut self) -> &mut aiRecord;

    fn val(&self) -> f64 {
        unsafe { self.as_raw_ai() }.val
    }
    fn set_val(&mut self, val: f64) {
        unsafe { self.as_raw_ai_mut() }.val = val;
    }
}

impl AiRecord for aiRecord {
    unsafe fn as_raw_ai(&self) -> &aiRecord {
        self
    }
    unsafe fn as_raw_ai_mut(&mut self) -> &mut aiRecord {
        self
    }
}
impl Record for aiRecord {
    type Private = AiPrivate;
    unsafe fn as_raw(&self) -> &dbCommon {
        (self as *const _ as *const dbCommon).as_ref().unwrap()
    }
    unsafe fn as_raw_mut(&mut self) -> &mut dbCommon {
        (self as *mut _ as *mut dbCommon).as_mut().unwrap()
    }
}

impl<'a> Into<AnyRecordRef<'a>> for &'a mut aiRecord {
    fn into(self) -> AnyRecordRef<'a> {
        AnyRecordRef::Ai(self)
    }
}


pub struct AoPrivate {
    base: Private,
    handler: Box<dyn AoHandler + Send>,
}

impl Deref for AoPrivate {
    type Target = Private;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for AoPrivate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// Analog output record
pub trait AoRecord: Record<Private=AoPrivate> {
    unsafe fn as_raw_ao(&self) -> &aoRecord;
    unsafe fn as_raw_ao_mut(&mut self) -> &mut aoRecord;

    fn val(&self) -> f64 {
        unsafe { self.as_raw_ao() }.val
    }
    fn set_val(&mut self, val: f64) {
        unsafe { self.as_raw_ao_mut() }.val = val;
    }
}

impl AoRecord for aoRecord {
    unsafe fn as_raw_ao(&self) -> &aoRecord {
        self
    }
    unsafe fn as_raw_ao_mut(&mut self) -> &mut aoRecord {
        self
    }
}
impl Record for aoRecord {
    type Private = AoPrivate;
    unsafe fn as_raw(&self) -> &dbCommon {
        (self as *const _ as *const dbCommon).as_ref().unwrap()
    }
    unsafe fn as_raw_mut(&mut self) -> &mut dbCommon {
        (self as *mut _ as *mut dbCommon).as_mut().unwrap()
    }
}

impl<'a> Into<AnyRecordRef<'a>> for &'a mut aoRecord {
    fn into(self) -> AnyRecordRef<'a> {
        AnyRecordRef::Ao(self)
    }
}
