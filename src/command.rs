use std::slice;
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Mutex;
use std::collections::HashMap;

use libc::{c_int};

use lazy_static::lazy_static;

use epics_sys::{
    self,
    iocshRegister, iocshFuncDef,
    iocshArg, iocshArgType, iocshArgBuf,
};

lazy_static! {
    static ref COMMANDS: Mutex<HashMap<CString, FuncDef>> = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
pub struct FuncDef {
    raw: Box<iocshFuncDef>,
    raw_args: Vec<*const iocshArg>,
    name: CString,
    args: Vec<ArgDef>,
}
impl FuncDef {
    pub fn new(name: &str) -> Self {
        let name = CString::new(name.to_string()).unwrap();
        Self {
            raw: Box::new(iocshFuncDef {
                name: name.as_c_str().as_ptr(),
                nargs: 0,
                arg: ptr::null(),
            }),
            raw_args: Vec::new(),
            name, args: Vec::new(),
        }
    }
    pub fn arg(mut self, name: &str, dtype: ArgType) -> Self {
        self.args.push(ArgDef::new(name, dtype));
        self
    }
    pub fn register(mut self, f: extern "C" fn(*const iocshArgBuf)) {
        for arg in self.args.iter() {
            self.raw_args.push(arg.as_raw() as *const _);
        }
        self.raw.nargs = self.args.len() as c_int;
        self.raw.arg = self.raw_args.as_ptr();
        unsafe { iocshRegister(self.raw.as_ref() as *const _, Some(f)) };
        assert!(COMMANDS.lock().unwrap().insert(self.name.clone(), self).is_none());
    }
}
unsafe impl Send for FuncDef {}

#[allow(dead_code)]
pub struct ArgDef {
    raw: iocshArg,
    name: CString,
    dtype: ArgType,
}
impl ArgDef {
    pub fn new(name: &str, dtype: ArgType) -> Self {
        let name = CString::new(name.to_string()).unwrap();
        Self {
            raw: iocshArg {
                name: name.as_c_str().as_ptr(),
                type_: dtype.as_raw(),
            },
            name, dtype,
        }
    }
    fn as_raw(&self) -> &iocshArg {
        &self.raw
    }
}
unsafe impl Send for ArgDef {}

pub enum ArgType {
    Int,
    Double,
    String,
    PersistentString,
    Pdbbase,
    Argv,
}
impl ArgType {
    pub fn as_raw(&self) -> iocshArgType {
        match self {
            ArgType::Int              => epics_sys::iocshArgType_iocshArgInt,
            ArgType::Double           => epics_sys::iocshArgType_iocshArgDouble,
            ArgType::String           => epics_sys::iocshArgType_iocshArgString,
            ArgType::PersistentString => epics_sys::iocshArgType_iocshArgPdbbase,
            ArgType::Pdbbase          => epics_sys::iocshArgType_iocshArgArgv,
            ArgType::Argv             => epics_sys::iocshArgType_iocshArgPersistentString,
        }
    }
}

pub trait AsType {
    fn dtype() -> ArgType;
    fn from_buf(buf: &iocshArgBuf) -> Self;
}

impl AsType for i32 {
    fn dtype() -> ArgType {
        ArgType::Int
    }
    fn from_buf(buf: &iocshArgBuf) -> Self {
        (unsafe { buf.ival }) as Self
    }
}
impl AsType for f64 {
    fn dtype() -> ArgType {
        ArgType::Double
    }
    fn from_buf(buf: &iocshArgBuf) -> Self {
        (unsafe { buf.dval }) as Self
    }
}
impl<'a> AsType for &'a str {
    fn dtype() -> ArgType {
        ArgType::String
    }
    fn from_buf(buf: &iocshArgBuf) -> Self {
        unsafe { CStr::from_ptr(buf.sval) }.to_str().unwrap()
    }
}
impl<'a> AsType for String {
    fn dtype() -> ArgType {
        ArgType::PersistentString
    }
    fn from_buf(buf: &iocshArgBuf) -> Self {
        unsafe { CString::from_raw(buf.sval) }.into_string().unwrap()
    }
}

pub struct ArgBuf<'a> {
    buffer: &'a [iocshArgBuf],
}
impl<'a> ArgBuf<'a> {
    pub fn new(buffer: *const iocshArgBuf, len: usize) -> Self {
        Self { buffer: unsafe { slice::from_raw_parts(buffer, len) } }
    }
    pub fn iter(&self) -> slice::Iter<'a, iocshArgBuf> {
        self.buffer.iter()
    }
}

#[macro_export]
macro_rules! _replace {
    ($_t:tt, $sub:expr) => {$sub};
}

#[macro_export]
macro_rules! register_command {
    ( fn $fn_name:ident ( $( $arg_name:ident : $arg_type:ty ),* ) $fn_body:block ) => {
        register_command!(fn $fn_name ( $( $arg_name : $arg_type ),* ) -> () $fn_body);
    };
    ( fn $fn_name:ident ( $( $arg_name:ident : $arg_type:ty ),* ) -> () $fn_body:block ) => {
        extern "C" fn wrapper(args: *const $crate::epics_sys::iocshArgBuf) {
            fn user_func( $( $arg_name : $arg_type ),* ) $fn_body
            let len = {<[()]>::len(&[ $( $crate::_replace!($arg_type, ()) ),* ])};
            let arg_buf = $crate::command::ArgBuf::new(args, len);
            let mut iter = arg_buf.iter();
            user_func($(
                <$arg_type as $crate::command::AsType>::from_buf(iter.next().unwrap())
            ),*);
        }
        $crate::command::FuncDef::new(stringify!($fn_name))
        $(
            .arg(stringify!($fn_name), <$arg_type as $crate::command::AsType>::dtype())
        )*
        .register(wrapper);
    };
}
