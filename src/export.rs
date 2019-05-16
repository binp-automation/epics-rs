#![allow(dead_code)]
#![allow(unused_variables)]

use libc::{c_int, c_long};

use crate::epics::{
//    iocshRegister, iocshFuncDef, iocshArg,
    dbCommon, aiRecord, aoRecord, biRecord, boRecord,
};

use crate::record::{Record, AiRecord, AoRecord, BiRecord, BoRecord};

#[no_mangle]
extern fn rsbind_init() -> c_long {
    println!("[rsbind] init");
    crate::system::init();
    0
}

#[no_mangle]
extern fn rsbind_quit() -> c_long {
    println!("[rsbind] quit");
    crate::system::quit();
    0
}

// any record

#[no_mangle]
extern fn rsbind_get_ioint_info(cmd: c_int, rec: *mut dbCommon) -> c_long {
    let r = Record::from(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] get_ioint_info({})", r.name());
    0
}

// ai record

#[no_mangle]
extern fn rsbind_ai_init_record(rec: *mut aiRecord) -> c_long {
    let r = AiRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] ai_init_record({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rsbind_ai_read_ai(rec: *mut aiRecord) -> c_long {
    let r = AiRecord::from(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] ai_read_ai({})", r.name());
    0
}
#[no_mangle]
extern fn rsbind_ai_special_linconv(rec: *mut aiRecord, after: c_int) -> c_long {
    let r = AiRecord::from(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] ai_special_linconv({})", r.name());
    0
}

// ao record

#[no_mangle]
extern fn rsbind_ao_init_record(rec: *mut aoRecord) -> c_long {
    let r = AoRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] ao_init_record({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rsbind_ao_write_ao(rec: *mut aoRecord) -> c_long {
    let r = AoRecord::from(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] ao_write_ao({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rsbind_ao_special_linconv(rec: *mut aoRecord, after: c_int) -> c_long {
    let r = AoRecord::from(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] ao_special_linconv({})", r.name());
    0
}

// bi record

#[no_mangle]
extern fn rsbind_bi_init_record(rec: *mut biRecord) -> c_long {
    let r = BiRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] bi_init_record({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rsbind_bi_read_bi(rec: *mut biRecord, after: c_int) -> c_long {
    let r = BiRecord::from(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] bi_read_bi({})", r.name());
    0
}

// bo record

#[no_mangle]
extern fn rsbind_bo_init_record(rec: *mut boRecord) -> c_long {
    let r = BoRecord::new(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] bo_init_record({}): {}", r.name(), r.val());
    0
}
#[no_mangle]
extern fn rsbind_bo_write_bo(rec: *mut boRecord, after: c_int) -> c_long {
    let r = BoRecord::from(unsafe { rec.as_mut().unwrap() });
    println!("[rsbind] bo_write_bo({}): {}", r.name(), r.val());
    0
}
