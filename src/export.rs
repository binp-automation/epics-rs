#![allow(dead_code)]
#![allow(unused_variables)]

use libc::{c_int, c_long};

use crate::epics::{
//    iocshRegister, iocshFuncDef, iocshArg,
    IOSCANPVT,
    dbCommon, aiRecord, aoRecord, biRecord, boRecord,
};
use crate::record::{Record, AiRecord, AoRecord, BiRecord, BoRecord};
use crate::system;

#[macro_export]
macro_rules! bind_device_support {
    ($dsmf:expr) => {
        #[no_mangle]
        extern fn rsbind_init() {
            $crate::system::init(Box::new($dsmf()));
        }
    }
}

#[no_mangle]
extern fn rsbind_quit() {
    system::quit();
}

// any record

#[no_mangle]
extern fn rsbind_get_ioint_info(detach: c_int, rec: *mut dbCommon, ppvt: *mut IOSCANPVT) -> c_long {
    let record = Record::from(unsafe { rec.as_mut().unwrap() });
    unsafe { *ppvt = system::record_ioint(detach != 0, record).raw; }
    0
}

// ai record

#[no_mangle]
extern fn rsbind_ai_init_record(rec: *mut aiRecord) -> c_long {
    system::record_init(AiRecord::new(unsafe { rec.as_mut().unwrap() }).into());
    0
}
#[no_mangle]
extern fn rsbind_ai_read_ai(rec: *mut aiRecord) -> c_long {
    system::record_read(AiRecord::from(unsafe { rec.as_mut().unwrap() }).into());
    0
}
#[no_mangle]
extern fn rsbind_ai_special_linconv(rec: *mut aiRecord, after: c_int) -> c_long {
    //AiRecord::from(unsafe { rec.as_mut().unwrap() });
    0
}

// ao record

#[no_mangle]
extern fn rsbind_ao_init_record(rec: *mut aoRecord) -> c_long {
    system::record_init(AoRecord::new(unsafe { rec.as_mut().unwrap() }).into());
    0
}
#[no_mangle]
extern fn rsbind_ao_write_ao(rec: *mut aoRecord) -> c_long {
    system::record_write(AoRecord::from(unsafe { rec.as_mut().unwrap() }).into());
    0
}
#[no_mangle]
extern fn rsbind_ao_special_linconv(rec: *mut aoRecord, after: c_int) -> c_long {
    // AoRecord::from(unsafe { rec.as_mut().unwrap() });
    0
}

// bi record

#[no_mangle]
extern fn rsbind_bi_init_record(rec: *mut biRecord) -> c_long {
    system::record_init(BiRecord::new(unsafe { rec.as_mut().unwrap() }).into());
    0
}
#[no_mangle]
extern fn rsbind_bi_read_bi(rec: *mut biRecord, after: c_int) -> c_long {
    system::record_read(BiRecord::from(unsafe { rec.as_mut().unwrap() }).into());
    0
}

// bo record

#[no_mangle]
extern fn rsbind_bo_init_record(rec: *mut boRecord) -> c_long {
    system::record_init(BoRecord::new(unsafe { rec.as_mut().unwrap() }).into());
    0
}
#[no_mangle]
extern fn rsbind_bo_write_bo(rec: *mut boRecord, after: c_int) -> c_long {
    system::record_write(BoRecord::from(unsafe { rec.as_mut().unwrap() }).into());
    0
}
