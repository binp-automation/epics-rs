use std::convert::{TryFrom};
use std::ops::{Deref, DerefMut};

use super::{
    Record, ReadRecord, WriteRecord,
    AiRecord, AoRecord, AiHandler, AoHandler,
    BiRecord, BoRecord, BiHandler, BoHandler,
    LonginRecord, LongoutRecord, LonginHandler, LongoutHandler,
    StringinRecord, StringoutRecord, StringinHandler, StringoutHandler,
};

/// Record type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordType {
    Ai,
    Ao,
    Bi,
    Bo,
    Longin,
    Longout,
    Stringin,
    Stringout,
}

macro_rules! into_any {
    ($any:ident, $opt:ident, $type:ty) => {
        impl Into<$any> for $type {
            fn into(self) -> $any {
                $any::$opt(self)
            }
        }
    };
}

macro_rules! try_from_any {
    ($any:ident, $opt:ident, $type:ty) => {
        impl TryFrom<$any> for $type {
            type Error = $any;
            fn try_from(a: $any) -> Result<Self, $any> {
                if let $any::$opt(x) = a {
                    Ok(x)
                } else {
                    Err(a)
                }
            }
        }
        impl<'a> TryFrom<&'a mut $any> for &'a mut $type {
            type Error = ();
            fn try_from(a: &'a mut $any) -> Result<Self, ()> {
                if let $any::$opt(ref mut x) = a {
                    Ok(x)
                } else {
                    Err(())
                }
            }
        }
    };
}

macro_rules! try_set_handler {
    ($any:ident, $rec:ident, $Handler:ident) => {
        Box::<dyn $Handler + Send>::try_from($any)
        .map_err(|_| 1).and_then(|hdl| {
            match $rec.replace_handler(hdl) {
                Some(_) => Err(2),
                None => Ok(()),
            }
        })
    };
}

/// Any record wrapper - could contain any record
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
impl AnyRecord {
    pub fn rtype(&self) -> RecordType {
        match self {
            AnyRecord::Ai(_) => RecordType::Ai,
            AnyRecord::Ao(_) => RecordType::Ao,
            AnyRecord::Bi(_) => RecordType::Bi,
            AnyRecord::Bo(_) => RecordType::Bo,
            AnyRecord::Longin(_) => RecordType::Longin,
            AnyRecord::Longout(_) => RecordType::Longout,
            AnyRecord::Stringin(_) => RecordType::Stringin,
            AnyRecord::Stringout(_) => RecordType::Stringout,
        }
    }
    pub unsafe fn try_set_handler(&mut self, any: AnyHandlerBox)
    -> Result<(), crate::Error> {
        let any_type = any.rtype();
        match self {
            AnyRecord::Ai(ref mut rec) => try_set_handler!(any, rec, AiHandler),
            AnyRecord::Ao(ref mut rec) => try_set_handler!(any, rec, AoHandler),
            AnyRecord::Bi(ref mut rec) => try_set_handler!(any, rec, BiHandler),
            AnyRecord::Bo(ref mut rec) => try_set_handler!(any, rec, BoHandler),
            AnyRecord::Longin(ref mut rec) => try_set_handler!(any, rec, LonginHandler),
            AnyRecord::Longout(ref mut rec) => try_set_handler!(any, rec, LongoutHandler),
            AnyRecord::Stringin(ref mut rec) => try_set_handler!(any, rec, StringinHandler),
            AnyRecord::Stringout(ref mut rec) => try_set_handler!(any, rec, StringoutHandler),
        }.map_err(|n| {
            match n {
                1 => crate::Error::Other(format!(
                    "record and handler type mismatch: {:?} != {:?}",
                    self.rtype(), any_type,
                ).into()),
                2 => crate::Error::Other("handler already set".into()),
                _ => unreachable!(),
            }
        })
    }
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

into_any!(AnyRecord, Ai, AiRecord);
into_any!(AnyRecord, Ao, AoRecord);
into_any!(AnyRecord, Bi, BiRecord);
into_any!(AnyRecord, Bo, BoRecord);
into_any!(AnyRecord, Longin, LonginRecord);
into_any!(AnyRecord, Longout, LongoutRecord);
into_any!(AnyRecord, Stringin, StringinRecord);
into_any!(AnyRecord, Stringout, StringoutRecord);

try_from_any!(AnyRecord, Ai, AiRecord);
try_from_any!(AnyRecord, Ao, AoRecord);
try_from_any!(AnyRecord, Bi, BiRecord);
try_from_any!(AnyRecord, Bo, BoRecord);
try_from_any!(AnyRecord, Longin, LonginRecord);
try_from_any!(AnyRecord, Longout, LongoutRecord);
try_from_any!(AnyRecord, Stringin, StringinRecord);
try_from_any!(AnyRecord, Stringout, StringoutRecord);


/// Any readable record wrapper
pub enum AnyReadRecord {
    Ai(AiRecord),
    Bi(BiRecord),
    Longin(LonginRecord),
    Stringin(StringinRecord),
}
impl AnyReadRecord {
    pub fn rtype(&self) -> RecordType {
        match self {
            AnyReadRecord::Ai(_) => RecordType::Ai,
            AnyReadRecord::Bi(_) => RecordType::Bi,
            AnyReadRecord::Longin(_) => RecordType::Longin,
            AnyReadRecord::Stringin(_) => RecordType::Stringin,
        }
    }
}
impl Deref for AnyReadRecord {
    type Target = ReadRecord;
    fn deref(&self) -> &Self::Target {
        match self {
            AnyReadRecord::Ai(ref r) => r,
            AnyReadRecord::Bi(ref r) => r,
            AnyReadRecord::Longin(ref r) => r,
            AnyReadRecord::Stringin(ref r) => r,
        }
    }
}
impl DerefMut for AnyReadRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            AnyReadRecord::Ai(ref mut r) => r,
            AnyReadRecord::Bi(ref mut r) => r,
            AnyReadRecord::Longin(ref mut r) => r,
            AnyReadRecord::Stringin(ref mut r) => r,
        }
    }
}

into_any!(AnyReadRecord, Ai, AiRecord);
into_any!(AnyReadRecord, Bi, BiRecord);
into_any!(AnyReadRecord, Longin, LonginRecord);
into_any!(AnyReadRecord, Stringin, StringinRecord);

try_from_any!(AnyReadRecord, Ai, AiRecord);
try_from_any!(AnyReadRecord, Bi, BiRecord);
try_from_any!(AnyReadRecord, Longin, LonginRecord);
try_from_any!(AnyReadRecord, Stringin, StringinRecord);


/// Any writable record wrapper
pub enum AnyWriteRecord {
    Ao(AoRecord),
    Bo(BoRecord),
    Longout(LongoutRecord),
    Stringout(StringoutRecord),
}
impl AnyWriteRecord {
    pub fn rtype(&self) -> RecordType {
        match self {
            AnyWriteRecord::Ao(_) => RecordType::Ao,
            AnyWriteRecord::Bo(_) => RecordType::Bo,
            AnyWriteRecord::Longout(_) => RecordType::Longout,
            AnyWriteRecord::Stringout(_) => RecordType::Stringout,
        }
    }
}
impl Deref for AnyWriteRecord {
    type Target = WriteRecord;
    fn deref(&self) -> &Self::Target {
        match self {
            AnyWriteRecord::Ao(ref r) => r,
            AnyWriteRecord::Bo(ref r) => r,
            AnyWriteRecord::Longout(ref r) => r,
            AnyWriteRecord::Stringout(ref r) => r,
        }
    }
}
impl DerefMut for AnyWriteRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            AnyWriteRecord::Ao(ref mut r) => r,
            AnyWriteRecord::Bo(ref mut r) => r,
            AnyWriteRecord::Longout(ref mut r) => r,
            AnyWriteRecord::Stringout(ref mut r) => r,
        }
    }
}

into_any!(AnyWriteRecord, Ao, AoRecord);
into_any!(AnyWriteRecord, Bo, BoRecord);
into_any!(AnyWriteRecord, Longout, LongoutRecord);
into_any!(AnyWriteRecord, Stringout, StringoutRecord);

try_from_any!(AnyWriteRecord, Ao, AoRecord);
try_from_any!(AnyWriteRecord, Bo, BoRecord);
try_from_any!(AnyWriteRecord, Longout, LongoutRecord);
try_from_any!(AnyWriteRecord, Stringout, StringoutRecord);


/// Any boxed handler wrapper
pub enum AnyHandlerBox {
    Ai(Box<dyn AiHandler + Send>),
    Ao(Box<dyn AoHandler + Send>),
    Bi(Box<dyn BiHandler + Send>),
    Bo(Box<dyn BoHandler + Send>),
    Longin(Box<dyn LonginHandler + Send>),
    Longout(Box<dyn LongoutHandler + Send>),
    Stringin(Box<dyn StringinHandler + Send>),
    Stringout(Box<dyn StringoutHandler + Send>),
}
impl AnyHandlerBox {
    pub fn rtype(&self) -> RecordType {
        match self {
            AnyHandlerBox::Ai(_) => RecordType::Ai,
            AnyHandlerBox::Ao(_) => RecordType::Ao,
            AnyHandlerBox::Bi(_) => RecordType::Bi,
            AnyHandlerBox::Bo(_) => RecordType::Bo,
            AnyHandlerBox::Longin(_) => RecordType::Longin,
            AnyHandlerBox::Longout(_) => RecordType::Longout,
            AnyHandlerBox::Stringin(_) => RecordType::Stringin,
            AnyHandlerBox::Stringout(_) => RecordType::Stringout,
        }
    }
}

into_any!(AnyHandlerBox, Ai, Box<dyn AiHandler + Send>);
into_any!(AnyHandlerBox, Ao, Box<dyn AoHandler + Send>);
into_any!(AnyHandlerBox, Bi, Box<dyn BiHandler + Send>);
into_any!(AnyHandlerBox, Bo, Box<dyn BoHandler + Send>);
into_any!(AnyHandlerBox, Longin, Box<dyn LonginHandler + Send>);
into_any!(AnyHandlerBox, Longout, Box<dyn LongoutHandler + Send>);
into_any!(AnyHandlerBox, Stringin, Box<dyn StringinHandler + Send>);
into_any!(AnyHandlerBox, Stringout, Box<dyn StringoutHandler + Send>);

try_from_any!(AnyHandlerBox, Ai, Box<dyn AiHandler + Send>);
try_from_any!(AnyHandlerBox, Ao, Box<dyn AoHandler + Send>);
try_from_any!(AnyHandlerBox, Bi, Box<dyn BiHandler + Send>);
try_from_any!(AnyHandlerBox, Bo, Box<dyn BoHandler + Send>);
try_from_any!(AnyHandlerBox, Longin, Box<dyn LonginHandler + Send>);
try_from_any!(AnyHandlerBox, Longout, Box<dyn LongoutHandler + Send>);
try_from_any!(AnyHandlerBox, Stringin, Box<dyn StringinHandler + Send>);
try_from_any!(AnyHandlerBox, Stringout, Box<dyn StringoutHandler + Send>);
