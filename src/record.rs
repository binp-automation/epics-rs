use std::ops::{Deref, DerefMut};
use std::ptr;

use libc::{c_int, c_ushort, c_char, c_void};

use epics_sys::{
    IOSCANPVT, scanIoInit, scanIoRequest,
    CALLBACK, callbackSetProcess, callbackRequest,
};
use epics_sys::{
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
    raw: IOSCANPVT,
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
    pub unsafe fn as_raw(&self) -> &IOSCANPVT {
        &self.raw
    }
}
unsafe impl Send for Scan {}
unsafe impl Sync for Scan {}

pub trait Record: Deref<Target=RecordBase> + DerefMut {
    type Raw;
    unsafe fn from_raw(raw: Self::Raw) -> Self;
    unsafe fn init_raw(&mut self);
}


#[derive(Debug)]
pub struct Private {
    callback: CALLBACK,
    scan: Scan,
}

/// Common EPICS record
pub struct RecordBase {
    raw: &'static mut dbCommon,
}

impl RecordBase {
    pub unsafe fn from_raw(raw: *mut dbCommon) -> Self {
        Self { raw: raw.as_mut().unwrap() }
    }
    pub unsafe fn init_raw(&mut self) {
        let mut cb: CALLBACK = CALLBACK::default();
        callbackSetProcess(
            (&mut cb) as *mut _,
            self.raw.prio as c_int,
            self.raw as *mut _ as *mut c_void,
        );
        let priv_data = Private { callback: cb, scan: Scan::new() };
        self.raw.dpvt = Box::leak(Box::new(priv_data)) as *mut _ as *mut c_void;
    }

    pub fn name(&self) -> &[u8] {
        cstr_to_slice(&self.raw.name)
    }

    pub unsafe fn pact(&self) -> bool {
        self.raw.pact != 0
    }
    pub unsafe fn set_pact(&mut self, pact: bool) {
        self.raw.pact = if pact { 1 } else { 0 };
    }

    #[allow(dead_code)]
    pub unsafe fn private(&self) -> &Private {
        let ptr = self.raw.dpvt as *const Private;
        ptr.as_ref().unwrap()
    }
    pub unsafe fn private_mut(&mut self) -> &mut Private {
        let ptr = self.raw.dpvt as *mut Private;
        ptr.as_mut().unwrap()
    }

    pub unsafe fn process(&mut self) {
        let priv_data = self.private_mut();
        let cb = &mut priv_data.callback;
        assert_eq!(callbackRequest(cb as *mut _), 0);
    }

    pub unsafe fn get_scan(&self) -> &Scan {
        &self.private().scan
    }
}

/// ai record
pub struct AiRecord {
    raw: &'static mut aiRecord,
    base: RecordBase,
}

impl Record for AiRecord {
    type Raw = *mut aiRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        let ptr = raw as *mut dbCommon;
        let base = RecordBase::from_raw(ptr);
        Self { raw: raw.as_mut().unwrap(), base }
    }
    unsafe fn init_raw(&mut self) {
        self.base.init_raw();
    }
}
impl AiRecord {
    pub fn val(&self) -> f64 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: f64) {
        self.raw.val = val;
    }
}

impl Deref for AiRecord {
    type Target = RecordBase;
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

/// ao record
pub struct AoRecord {
    raw: &'static mut aoRecord,
    base: RecordBase,
}

impl Record for AoRecord {
    type Raw = *mut aoRecord;
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        let ptr = raw as *mut dbCommon;
        let base = RecordBase::from_raw(ptr);
        Self { raw: raw.as_mut().unwrap(), base }
    }
    unsafe fn init_raw(&mut self) {
        self.base.init_raw();
    }
}
impl AoRecord {
    pub fn val(&self) -> f64 {
        self.raw.val
    }
    pub fn set_val(&mut self, val: f64) {
        self.raw.val = val;
    }
}

impl Deref for AoRecord {
    type Target = RecordBase;
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

/// bi record
pub struct BiRecord {
    raw: &'static mut biRecord,
    base: RecordBase,
}

impl Record for BiRecord {
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

/// bo record
pub struct BoRecord {
    raw: &'static mut boRecord,
    base: RecordBase,
}

impl Record for BoRecord {
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

/// longin record
pub struct LonginRecord {
    raw: &'static mut longinRecord,
    base: RecordBase,
}

impl Record for LonginRecord {
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

/// longout record
pub struct LongoutRecord {
    raw: &'static mut longoutRecord,
    base: RecordBase,
}

impl Record for LongoutRecord {
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


/// stringin record
pub struct StringinRecord {
    raw: &'static mut stringinRecord,
    base: RecordBase,
}

impl Record for StringinRecord {
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
        copy_cstr_from_slice(&mut self.raw.val, val);
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

/// stringout record
pub struct StringoutRecord {
    raw: &'static mut stringoutRecord,
    base: RecordBase,
}

impl Record for StringoutRecord {
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
        copy_cstr_from_slice(&mut self.raw.val, val);
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
    type Target = RecordBase;
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
    type Target = RecordBase;
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
    type Target = RecordBase;
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
    type Target = RecordBase;
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
