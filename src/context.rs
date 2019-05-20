use std::slice;
use std::ffi::{CStr, CString};
//use std::sync::Mutex;
//use std::collections::HashMap;

//use lazy_static::lazy_static;

use crate::epics::{
    self,
    iocshRegister, iocshFuncDef,
    iocshArg, iocshArgType, iocshArgBuf,
};
/*
lazy_static! {
    static ref COMMANDS: Mutex<HashMap<String, FuncDef>> = Mutex::new(HashMap::new());
}
*/
pub struct FuncDef {
    name: CString,
    args: Vec<ArgDef>,

}
impl FuncDef {
    pub fn new(name: &str) -> Self {
        Self { name: CString::new(name.to_string()).unwrap(), args: Vec::new() }
    }
    pub fn arg(mut self, name: &str, dtype: ArgType) -> Self {
        self.args.push(ArgDef::new(name, dtype));
        self
    }
    pub fn register(self, f: extern "C" fn(*const iocshArgBuf)) {
        let raw_args: Vec<_> = self.args.iter().map(|a| a.as_raw_ptr()).collect();
        let raw = iocshFuncDef {
            name: self.name.as_c_str().as_ptr(),
            nargs: raw_args.len() as i32,
            arg: raw_args.as_ptr(),
        };
        println!("{:?}", raw);
        println!("{:?}", unsafe { CStr::from_ptr(raw.name) });
        unsafe { iocshRegister(&raw as *const _, Some(f)) };
    }
}

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
    fn as_raw_ptr(&self) -> *const iocshArg {
        &self.raw as *const _
    }
}

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
            ArgType::Int              => epics::iocshArgType_iocshArgInt,
            ArgType::Double           => epics::iocshArgType_iocshArgDouble,
            ArgType::String           => epics::iocshArgType_iocshArgString,
            ArgType::PersistentString => epics::iocshArgType_iocshArgPdbbase,
            ArgType::Pdbbase          => epics::iocshArgType_iocshArgArgv,
            ArgType::Argv             => epics::iocshArgType_iocshArgPersistentString,
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
        extern "C" fn wrapper(args: *const $crate::epics::iocshArgBuf) {
            fn user_func( $( $arg_name : $arg_type ),* ) $fn_body
            let len = {<[()]>::len(&[$($crate::_replace!($arg_type, ())),*])};
            let arg_buf = $crate::context::ArgBuf::new(args, len);
            let mut iter = arg_buf.iter();
            user_func($(
                <$arg_type as $crate::context::AsType>::from_buf(iter.next().unwrap())
            ),*);
        }
        $crate::context::FuncDef::new(stringify!($fn_name))
        $(
            .arg(stringify!($fn_name), <$arg_type as $crate::context::AsType>::dtype())
        )*
        .register(wrapper);
    };
}
