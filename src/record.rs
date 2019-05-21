use std::ops::{Deref, DerefMut};
use std::ptr;

use libc::{c_int, c_ushort, c_char, c_void};

use crate::epics::{
    IOSCANPVT, scanIoInit, scanIoRequest,
    CALLBACK, callbackSetProcess, callbackRequest,
};
use crate::epics::{
    dbCommon,
    aiRecord, aoRecord,
    biRecord, boRecord,
    longinRecord, longoutRecord,
    stringinRecord, stringoutRecord,
};

fn copy_cstr_from_slice(dst: &mut [c_char], src: &[u8]) {
    let maxlen = dst.len() - 1;
    let src = if src.len() > maxlen {
        &src[..maxlen]
    } else {
        src
    };
    let src = unsafe{ &*( src as *const [u8] as *const [i8] ) };
    dst[..maxlen].copy_from_slice(src);
    dst[src.len() + 1] = b'\0' as i8;
}

fn cstr_to_slice(src: &[c_char]) -> &[u8] {
    let maxlen = src.len();
    let mut len = maxlen;
    for i in 0..maxlen {
        if unsafe { *src.get_unchecked(i) } == b'\0' as i8 {
            len = i;
            break;
        }
    }
    unsafe{ &*( &src[..len] as *const _ as *const [u8] ) }
}

#[derive(Debug, Clone)]
pub struct Scan {
    pub(crate) raw: IOSCANPVT,
}

impl Scan {
    pub fn new() -> Self {
        let mut scan = ptr::null_mut();
        unsafe { scanIoInit((&mut scan) as *mut _); }
        Self { raw: scan }
    }
    pub fn request(&self) -> Result<(),()> {
        match unsafe { scanIoRequest(self.raw) } {
            0 => Err(()),
            _ => Ok(()),
        }
    }
}
unsafe impl Send for Scan {}
unsafe impl Sync for Scan {}

#[derive(Debug)]
pub(crate) struct Private {
    callback: CALLBACK,
    scan: Scan,
}

/// Common EPICS record
pub struct Record {
    raw: &'static mut dbCommon,
}

impl Record {
    pub(crate) fn init(raw: &'static mut dbCommon) {
        let mut cb: CALLBACK = CALLBACK::default();
        unsafe { callbackSetProcess(
            (&mut cb) as *mut _,
            raw.prio as c_int,
            raw as *mut _ as *mut c_void,
        ); }
        let priv_data = Private { callback: cb, scan: Scan::new() };
        raw.dpvt = Box::leak(Box::new(priv_data)) as *mut _ as *mut c_void;
    }
    pub(crate) fn from(raw: &'static mut dbCommon) -> Self {
        Self { raw }
    }
    pub fn name(&self) -> &[u8] {
        cstr_to_slice(&self.raw.name)
    }

    pub(crate) fn pact(&self) -> bool {
        self.raw.pact != 0
    }
    pub(crate) fn set_pact(&mut self, pact: bool) {
        self.raw.pact = if pact { 1 } else { 0 };
    }

    #[allow(dead_code)]
    pub(crate) fn private(&self) -> &Private {
        let ptr = self.raw.dpvt as *const Private;
        unsafe { ptr.as_ref().unwrap() }
    }
    pub(crate) fn private_mut(&mut self) -> &mut Private {
        let ptr = self.raw.dpvt as *mut Private;
        unsafe { ptr.as_mut().unwrap() }
    }

    pub(crate) fn process(&mut self) {
        let priv_data = self.private_mut();
        let cb = &mut priv_data.callback;
        unsafe { assert_eq!(callbackRequest(cb as *mut _), 0); }
    }

    pub(crate) fn scan(&self) -> &Scan {
        &self.private().scan
    }
}
unsafe impl Send for Record {}

/// ai record
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
impl Into<AnyRecord> for AiRecord {
    fn into(self) -> AnyRecord {
        AnyRecord::Ai(self)
    }
}
impl Into<ReadRecord> for AiRecord {
    fn into(self) -> ReadRecord {
        ReadRecord::Ai(self)
    }
}
unsafe impl Send for AiRecord {}

/// ao record
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
impl Into<AnyRecord> for AoRecord {
    fn into(self) -> AnyRecord {
        AnyRecord::Ao(self)
    }
}
impl Into<WriteRecord> for AoRecord {
    fn into(self) -> WriteRecord {
        WriteRecord::Ao(self)
    }
}
unsafe impl Send for AoRecord {}

/// bi record
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
impl Into<AnyRecord> for BiRecord {
    fn into(self) -> AnyRecord {
        AnyRecord::Bi(self)
    }
}
impl Into<ReadRecord> for BiRecord {
    fn into(self) -> ReadRecord {
        ReadRecord::Bi(self)
    }
}
unsafe impl Send for BiRecord {}

/// bo record
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
impl Into<AnyRecord> for BoRecord {
    fn into(self) -> AnyRecord {
        AnyRecord::Bo(self)
    }
}
impl Into<WriteRecord> for BoRecord {
    fn into(self) -> WriteRecord {
        WriteRecord::Bo(self)
    }
}
unsafe impl Send for BoRecord {}

/// longin record
pub struct LonginRecord {
    raw: &'static mut longinRecord,
    base: Record,
}

impl LonginRecord {
    pub(crate) fn new(raw: &'static mut longinRecord) -> Self {
        let ptr = (raw as *mut longinRecord) as *mut dbCommon;
        Record::init(unsafe{ &mut *ptr });
        Self::from(raw)
    }
    pub(crate) fn from(raw: &'static mut longinRecord) -> Self {
        let ptr = (raw as *mut longinRecord) as *mut dbCommon;
        let base = Record::from(unsafe{ &mut *ptr });
        Self { raw, base }
    }
    pub fn val(&self) -> i32 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: i32) {
        self.raw.val = val as c_int;
    }
}

impl Deref for LonginRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for LonginRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Into<AnyRecord> for LonginRecord {
    fn into(self) -> AnyRecord {
        AnyRecord::Longin(self)
    }
}
impl Into<ReadRecord> for LonginRecord {
    fn into(self) -> ReadRecord {
        ReadRecord::Longin(self)
    }
}
unsafe impl Send for LonginRecord {}

/// longout record
pub struct LongoutRecord {
    raw: &'static mut longoutRecord,
    base: Record,
}

impl LongoutRecord {
    pub(crate) fn new(raw: &'static mut longoutRecord) -> Self {
        let ptr = (raw as *mut longoutRecord) as *mut dbCommon;
        Record::init(unsafe{ &mut *ptr });
        Self::from(raw)
    }
    pub(crate) fn from(raw: &'static mut longoutRecord) -> Self {
        let ptr = (raw as *mut longoutRecord) as *mut dbCommon;
        let base = Record::from(unsafe{ &mut *ptr });
        Self { raw, base }
    }
    pub fn val(&self) -> i32 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: i32) {
        self.raw.val = val as c_int;
    }
}

impl Deref for LongoutRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for LongoutRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Into<AnyRecord> for LongoutRecord {
    fn into(self) -> AnyRecord {
        AnyRecord::Longout(self)
    }
}
impl Into<WriteRecord> for LongoutRecord {
    fn into(self) -> WriteRecord {
        WriteRecord::Longout(self)
    }
}
unsafe impl Send for LongoutRecord {}


/// stringin record
pub struct StringinRecord {
    raw: &'static mut stringinRecord,
    base: Record,
}

impl StringinRecord {
    pub(crate) fn new(raw: &'static mut stringinRecord) -> Self {
        let ptr = (raw as *mut stringinRecord) as *mut dbCommon;
        Record::init(unsafe{ &mut *ptr });
        Self::from(raw)
    }
    pub(crate) fn from(raw: &'static mut stringinRecord) -> Self {
        let ptr = (raw as *mut stringinRecord) as *mut dbCommon;
        let base = Record::from(unsafe{ &mut *ptr });
        Self { raw, base }
    }
    pub fn val(&self) -> &[u8] {
        cstr_to_slice(&self.raw.val)
    }
    pub fn set_val(&mut self, val: &[u8]) {
        copy_cstr_from_slice(&mut self.raw.val, val);
    }
}

impl Deref for StringinRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for StringinRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Into<AnyRecord> for StringinRecord {
    fn into(self) -> AnyRecord {
        AnyRecord::Stringin(self)
    }
}
impl Into<ReadRecord> for StringinRecord {
    fn into(self) -> ReadRecord {
        ReadRecord::Stringin(self)
    }
}
unsafe impl Send for StringinRecord {}

/// stringout record
pub struct StringoutRecord {
    raw: &'static mut stringoutRecord,
    base: Record,
}

impl StringoutRecord {
    pub(crate) fn new(raw: &'static mut stringoutRecord) -> Self {
        let ptr = (raw as *mut stringoutRecord) as *mut dbCommon;
        Record::init(unsafe{ &mut *ptr });
        Self::from(raw)
    }
    pub(crate) fn from(raw: &'static mut stringoutRecord) -> Self {
        let ptr = (raw as *mut stringoutRecord) as *mut dbCommon;
        let base = Record::from(unsafe{ &mut *ptr });
        Self { raw, base }
    }
    pub fn val(&self) -> &[u8] {
        cstr_to_slice(&self.raw.val)
    }
    pub fn set_val(&mut self, val: &[u8]) {
        copy_cstr_from_slice(&mut self.raw.val, val);
    }
}

impl Deref for StringoutRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for StringoutRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Into<AnyRecord> for StringoutRecord {
    fn into(self) -> AnyRecord {
        AnyRecord::Stringout(self)
    }
}
impl Into<WriteRecord> for StringoutRecord {
    fn into(self) -> WriteRecord {
        WriteRecord::Stringout(self)
    }
}
unsafe impl Send for StringoutRecord {}

// any record
pub enum AnyRecord {
    Ai(AiRecord),
    Ao(AoRecord),
    Bi(BiRecord),
    Bo(BoRecord),
    Longin(LonginRecord),
    Longout(LongoutRecord),
    Stringin(StringinRecord),
    Stringout(StringoutRecord),
}
impl Deref for AnyRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        match self {
            AnyRecord::Ai(ref r) => r,
            AnyRecord::Ao(ref r) => r,
            AnyRecord::Bi(ref r) => r,
            AnyRecord::Bo(ref r) => r,
            AnyRecord::Longin(ref r) => r,
            AnyRecord::Longout(ref r) => r,
            AnyRecord::Stringin(ref r) => r,
            AnyRecord::Stringout(ref r) => r,
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
            AnyRecord::Longin(ref mut r) => r,
            AnyRecord::Longout(ref mut r) => r,
            AnyRecord::Stringin(ref mut r) => r,
            AnyRecord::Stringout(ref mut r) => r,
        }
    }
}

pub enum ReadRecord {
    Ai(AiRecord),
    Bi(BiRecord),
    Longin(LonginRecord),
    Stringin(StringinRecord),
}
impl Deref for ReadRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        match self {
            ReadRecord::Ai(ref r) => r,
            ReadRecord::Bi(ref r) => r,
            ReadRecord::Longin(ref r) => r,
            ReadRecord::Stringin(ref r) => r,

        }
    }
}
impl DerefMut for ReadRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ReadRecord::Ai(ref mut r) => r,
            ReadRecord::Bi(ref mut r) => r,
            ReadRecord::Longin(ref mut r) => r,
            ReadRecord::Stringin(ref mut r) => r,
        }
    }
}

pub enum WriteRecord {
    Ao(AoRecord),
    Bo(BoRecord),
    Longout(LongoutRecord),
    Stringout(StringoutRecord),
}
impl Deref for WriteRecord {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        match self {
            WriteRecord::Ao(ref r) => r,
            WriteRecord::Bo(ref r) => r,
            WriteRecord::Longout(ref r) => r,
            WriteRecord::Stringout(ref r) => r,
        }
    }
}
impl DerefMut for WriteRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            WriteRecord::Ao(ref mut r) => r,
            WriteRecord::Bo(ref mut r) => r,
            WriteRecord::Longout(ref mut r) => r,
            WriteRecord::Stringout(ref mut r) => r,
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
