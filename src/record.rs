use std::ops::{Deref, DerefMut};
use std::mem;
use std::ffi::{CStr};

use libc::{c_int, c_ushort, c_void};

use driver::{Data};

use crate::epics::{
    CALLBACK, callbackRequestProcessCallback,
    dbCommon, aiRecord, aoRecord, biRecord, boRecord,
};

pub(crate) struct PrivateData {
    callback: CALLBACK,
    data: Data,
}

impl PrivateData {
    pub(crate) fn new() -> Self {
        Self {
            callback: unsafe { mem::uninitialized() },
            data: Data::new(),
        }
    }
}

/// Common EPICS record
pub struct Record {
    raw: &'static mut dbCommon,
}

impl Record {
    pub(crate) fn init(raw: &'static mut dbCommon) {
        let priv_data = Box::new(PrivateData::new());
        raw.dpvt = Box::leak(priv_data) as *mut _ as *mut c_void;
    }
    pub(crate) fn from(raw: &'static mut dbCommon) -> Self {
        Self { raw }
    }
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.raw.name.as_ptr()) }.to_str().unwrap()
    }

    pub(crate) fn private_data(&self) -> &PrivateData {
        let ptr = self.raw.dpvt as *const PrivateData;
        unsafe { ptr.as_ref().unwrap() }
    }
    pub(crate) fn private_data_mut(&mut self) -> &mut PrivateData {
        let ptr = self.raw.dpvt as *mut PrivateData;
        unsafe { ptr.as_mut().unwrap() }
    }
    pub(crate) fn process(&mut self) {
        let priv_data = self.private_data_mut();
        unsafe { callbackRequestProcessCallback(
            (&mut priv_data.callback) as *mut CALLBACK,
            self.raw.prio as c_int,
            (&mut self.raw) as *mut _ as *mut c_void,
        ); }
    }
}
unsafe impl Send for Record {}

/// Analog input record
pub struct AiRecord {
    raw: &'static mut aiRecord,
    base: Record,
}

impl AiRecord {
    pub(crate) fn new(raw: &'static mut aiRecord) -> Self {
        let ptr = (raw as *mut aiRecord) as *mut dbCommon;
        Record::init(unsafe{ &mut *ptr });
        Self::from(raw)
    }
    pub(crate) fn from(raw: &'static mut aiRecord) -> Self {
        let ptr = (raw as *mut aiRecord) as *mut dbCommon;
        let base = Record::from(unsafe{ &mut *ptr });
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
impl DerefMut for AiRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
unsafe impl Send for AiRecord {}

/// Analog output record
pub struct AoRecord {
    raw: &'static mut aoRecord,
    base: Record,
}

impl AoRecord {
    pub(crate) fn new(raw: &'static mut aoRecord) -> Self {
        let ptr = (raw as *mut aoRecord) as *mut dbCommon;
        Record::init(unsafe{ &mut *ptr });
        Self::from(raw)
    }
    pub(crate) fn from(raw: &'static mut aoRecord) -> Self {
        let ptr = (raw as *mut aoRecord) as *mut dbCommon;
        let base = Record::from(unsafe{ &mut *ptr });
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
impl DerefMut for AoRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
unsafe impl Send for AoRecord {}

/// Binary input record
pub struct BiRecord {
    raw: &'static mut biRecord,
    base: Record,
}

impl BiRecord {
    pub(crate) fn new(raw: &'static mut biRecord) -> Self {
        let ptr = (raw as *mut biRecord) as *mut dbCommon;
        Record::init(unsafe{ &mut *ptr });
        Self::from(raw)
    }
    pub(crate) fn from(raw: &'static mut biRecord) -> Self {
        let ptr = (raw as *mut biRecord) as *mut dbCommon;
        let base = Record::from(unsafe{ &mut *ptr });
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
impl DerefMut for BiRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
unsafe impl Send for BiRecord {}

/// Binary output record
pub struct BoRecord {
    raw: &'static mut boRecord,
    base: Record,
}

impl BoRecord {
    pub(crate) fn new(raw: &'static mut boRecord) -> Self {
        let ptr = (raw as *mut boRecord) as *mut dbCommon;
        Record::init(unsafe{ &mut *ptr });
        Self::from(raw)
    }
    pub(crate) fn from(raw: &'static mut boRecord) -> Self {
        let ptr = (raw as *mut boRecord) as *mut dbCommon;
        let base = Record::from(unsafe{ &mut *ptr });
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
impl DerefMut for BoRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
unsafe impl Send for BoRecord {}

// any record

pub enum AnyRecord {
    Ai(AiRecord),
    Ao(AoRecord),
    Bi(BiRecord),
    Bo(BoRecord),
}
impl Deref for AnyRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        match self {
            AnyRecord::Ai(ref r) => r,
            AnyRecord::Ao(ref r) => r,
            AnyRecord::Bi(ref r) => r,
            AnyRecord::Bo(ref r) => r,
        }
    }
}
impl DerefMut for AnyRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            AnyRecord::Ai(ref mut r) => r,
            AnyRecord::Ao(ref mut r) => r,
            AnyRecord::Bi(ref mut r) => r,
            AnyRecord::Bo(ref mut r) => r,
        }
    }
}

pub enum ReadRecord {
    Ai(AiRecord),
    Bi(BiRecord),
}
impl Deref for ReadRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        match self {
            ReadRecord::Ai(ref r) => r,
            ReadRecord::Bi(ref r) => r,
        }
    }
}
impl DerefMut for ReadRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ReadRecord::Ai(ref mut r) => r,
            ReadRecord::Bi(ref mut r) => r,
        }
    }
}

pub enum WriteRecord {
    Ao(AoRecord),
    Bo(BoRecord),
}
impl Deref for WriteRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        match self {
            WriteRecord::Ao(ref r) => r,
            WriteRecord::Bo(ref r) => r,
        }
    }
}
impl DerefMut for WriteRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            WriteRecord::Ao(ref mut r) => r,
            WriteRecord::Bo(ref mut r) => r,
        }
    }
}


pub enum LinconvRecord {
    Ai(AiRecord),
    Ao(AoRecord),
}
impl Deref for LinconvRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        match self {
            LinconvRecord::Ai(ref r) => r,
            LinconvRecord::Ao(ref r) => r,
        }
    }
}
impl DerefMut for LinconvRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            LinconvRecord::Ai(ref mut r) => r,
            LinconvRecord::Ao(ref mut r) => r,
        }
    }
}
