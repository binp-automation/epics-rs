use std::io;
use std::ffi::{CStr};

use libc::{c_int, c_char};

use crate::epics::{
    iocshRegister, iocshFuncDef, iocshCallFunc,
    iocshArg, iocshArgBuf,
};

/*
fn register_command(&mut self) -> io::Result<()> {

    struct iocshFuncDef {
        const char *name;
        int nargs;
        const iocshArg * const *arg;
    };

    struct iocshArg {
        const char *name;
        iocshArgType type;
    }iocshArg;


    iocshRegister(const iocshFuncDef *piocshFuncDef, iocshCallFunc func);
    Ok(())
}
*/

struct FuncDef {
    name: String,
    args: Vec<Arg>,
}

struct Arg {
    name: String,
    dtype: DType,
}

enum DType {
    Int,
    Double,
    String,
    PersistentString,
    Pdbbase,
    Argv,
}

macro_rules! register_command {
    ( fn $fn_name:ident ( $( $arg_name:ident : $arg_type:ty ),* ) -> $ret_type:ty $fn_body:block ) => {
        println!("name: {}", stringify!($fn_name));
        println!("ret: {}", stringify!($ret_type));
        $(
            println!("arg: {}:{}", stringify!($arg_name), stringify!($arg_type));
        )*
        println!("body: {}", stringify!($fn_body));
    };
    ( fn $fn_name:ident ( $( $arg_name:ident : $arg_type:ty ),* ) $fn_body:block ) => {
        register_command!(fn $fn_name ( $( $arg_name : $arg_type ),* ) -> () $fn_body);
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn text() {
        register_command!(fn hello() { println!("hello"); });
        register_command!(fn add(a: i32, b: i32) -> i32 { a + b });
    }
}
