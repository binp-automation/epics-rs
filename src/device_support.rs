use std::panic;
use std::sync::atomic::{AtomicBool, Ordering, fence};

use log::{debug, error};
use simple_logger;

use lazy_static::lazy_static;

use epics_sys::{IOSCANPVT};

use crate::record::*;
use crate::Context;

use crate::async_proc;

use crate::util::lossy;

lazy_static! {
    static ref GATE: AtomicBool = AtomicBool::new(true);
}

fn overwrite_panic() {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        GATE.store(false, Ordering::SeqCst);
        
        let payload = panic_info.payload();
        let payload = match payload.downcast_ref::<String>() {
            Some(payload) => payload.clone(),
            None => match payload.downcast_ref::<&str>() {
                Some(payload) => String::from(*payload),
                None => String::new(),
            },
        };
        let location = match panic_info.location() {
            Some(location) => format!(" in file '{}' at line {}", location.file(), location.line()),
            None => String::new(),
        };
        error!("panic occured{}\n{}", location, payload);

        default_hook(panic_info);
    }));
}

pub fn check_gate() -> bool {
    GATE.load(Ordering::SeqCst)
}

pub unsafe fn init<F>(f: F) where F: Fn(&mut Context) -> crate::Result<()> {
    overwrite_panic();
    simple_logger::init().unwrap();
    async_proc::start_loop();
    let mut ctx = Context::new();
    match f(&mut ctx) {
        Ok(()) => {
            debug!("init");
        },
        Err(err) => {
            error!("init: {}", err);
            GATE.store(false, Ordering::SeqCst);
        },
    }
}

pub unsafe fn record_init<R, F>(raw: R::Raw, f: F, ret: i32) -> i32 where
R: Record + FromRaw + Into<AnyRecord>, 
F: Fn(&mut AnyRecord) -> crate::Result<AnyHandlerBox> {
    let mut rec = R::from_raw(raw).into();
    rec.init();
    //let mut ctx = Context::new();
    match f(&mut rec).and_then(|hdl| {
        rec.try_set_handler(hdl)
    }){
        Ok(()) => {
            debug!("record_init({})", lossy(rec.name()));
            ret
        },
        Err(e) => {
            error!("record_init({}): {}", lossy(rec.name()), e);
            1
        },
    }
}

pub unsafe fn record_set_scan<R>(
    detach: bool, raw: R::Raw, ppvt: *mut IOSCANPVT
) -> i32 where R: ScanRecord + FromRaw {
    let mut rec = R::from_raw(raw);
    if detach {
        panic!(
            "record '{}' was deleted from I/O event list: {}",
            lossy(rec.name()), "this action is not supported yet",
        );
    }
    let scan = rec.create_scan();
    *ppvt = *scan.as_raw();
    rec.set_scan(scan.clone());
    //let mut ctx = Context::new();
    match rec.handler_set_scan(scan).unwrap_or_else(|| {
        Err(crate::Error::Other("no handler".into()))
    }) {
        Ok(()) => {
            debug!("record_set_scan({})", lossy(rec.name()));
            0
        },
        Err(e) => {
            error!("record_set_scan({}): {}", lossy(rec.name()), e);
            1
        },
    }
}
pub unsafe fn record_read<R>(raw: R::Raw, ret: i32) -> i32
where R: ReadRecord + FromRaw + Into<AnyReadRecord> {
    let mut rec = R::from_raw(raw);
    if !rec.pact() {
        //let mut ctx = Context::new();
        match rec.handler_read().unwrap_or_else(|| {
            Err(crate::Error::Other("no handler".into()))
        }) {
            Ok(a) => {
                debug!("record_read({})", lossy(rec.name()));
                if !a {
                    rec.set_pact(true);
                    fence(Ordering::SeqCst);
                    async_proc::record_read(rec.into());
                }
                ret
            },
            Err(e) => {
                error!("record_read({}): {}", lossy(rec.name()), e);
                1
            },
        }
    } else {
        fence(Ordering::SeqCst);
        ret
    }
}

pub unsafe fn record_write<R>(raw: R::Raw) -> i32
where R: WriteRecord + FromRaw + Into<AnyWriteRecord> {
    let mut rec = R::from_raw(raw);
    if !rec.pact() {
        //let mut ctx = Context::new();
        match rec.handler_write().unwrap_or_else(|| {
            Err(crate::Error::Other("no handler".into()))
        }) {
            Ok(a) => {
                debug!("record_write({})", lossy(rec.name()));
                if !a {
                    rec.set_pact(true);
                    fence(Ordering::SeqCst);
                    async_proc::record_write(rec.into());
                }
                0
            },
            Err(e) => {
                error!("record_write({}): {}", lossy(rec.name()), e);
                1
            },
        }
    } else {
        fence(Ordering::SeqCst);
        0
    }
}

pub unsafe fn record_linconv<R>(raw: R::Raw, _after: i32) -> i32
where R: Record + FromRaw {
    let rec = R::from_raw(raw);
    panic!(
        "record '{}' unexpectedly requested linconv: {}",
        lossy(rec.name()), "this action is not supported yet",
    );
}

#[macro_export]
macro_rules! _bind_record_init {
    ($init:path, $raw:ident, $rec:ident, $xfn:ident) => {
        $crate::_bind_record_init!($init, $raw, $rec, $xfn, 0);
    };
    ($init:path, $raw:ident, $rec:ident, $xfn:ident, $ret:expr) => {
        #[no_mangle]
        extern fn $xfn(
            rec: *mut $crate::epics_sys::$raw,
        ) -> $crate::libc::c_long {
            if $crate::device_support::check_gate() {
                unsafe {
                    $crate::device_support::record_init::<
                        $crate::record::$rec, _
                    >(rec, $init, $ret) as $crate::libc::c_long
                }
            } else {
                1
            }
        }
    };
}
#[macro_export]
macro_rules! _bind_record_set_scan {
    ($raw:ident, $rec:ident, $xfn:ident) => {
        #[no_mangle]
        extern fn $xfn(
            detach: $crate::libc::c_int,
            rec: *mut $crate::epics_sys::$raw,
            ppvt: *mut $crate::epics_sys::IOSCANPVT,
        ) -> $crate::libc::c_long {
            if $crate::device_support::check_gate() {
                unsafe {
                    $crate::device_support::record_set_scan::<$rec>(
                        detach != 0, rec, ppvt
                    ) as $crate::libc::c_long
                }
            } else {
                1
            }
        }
    }
}
#[macro_export]
macro_rules! _bind_record_read {
    ($raw:ident, $rec:ident, $xfn:ident) => {
        $crate::_bind_record_read!($raw, $rec, $xfn, 0);
    };
    ($raw:ident, $rec:ident, $xfn:ident, $ret:expr) => {
        #[no_mangle]
        extern fn $xfn(
            rec: *mut $crate::epics_sys::$raw,
        ) -> $crate::libc::c_long {
            if $crate::device_support::check_gate() {
                unsafe { 
                    $crate::device_support::record_read::<$rec>(rec, $ret)
                    as $crate::libc::c_long
                }
            } else {
                1
            }
        }
    };
}
#[macro_export]
macro_rules! _bind_record_write {
    ($raw:ident, $rec:ident, $xfn:ident) => {
        #[no_mangle]
        extern fn $xfn(
            rec: *mut $crate::epics_sys::$raw,
        ) -> $crate::libc::c_long {
            if $crate::device_support::check_gate() {
                unsafe {
                    $crate::device_support::record_write::<$rec>(rec)
                    as $crate::libc::c_long
                }
            } else {
                1
            }
        }
    };
}
#[macro_export]
macro_rules! _bind_record_linconv {
    ($raw:ident, $rec:ident, $xfn:ident) => {
        #[no_mangle]
        extern fn $xfn(
            rec: *mut $crate::epics_sys::$raw,
            after: $crate::libc::c_int,
        ) -> $crate::libc::c_long {
            if $crate::device_support::check_gate() {
                unsafe {
                    $crate::device_support::record_linconv::<$rec>(
                        rec, after as i32
                    ) as $crate::libc::c_long
                }
            } else {
                1
            }
        }
    };
}

#[macro_export]
macro_rules! bind_device_support {
    ( $( $x:path ),* ) => {
        $crate::bind_device_support!( $( $x, )* );
    };
    (
        $init:path,
        $record_init:path,
    ) => {
        #[no_mangle]
        extern fn rsbind_init() {
            unsafe { $crate::device_support::init($init) };
        }

        // ai record
        $crate::_bind_record_init!($record_init, aiRecord, AiRecord, rsbind_ai_init_record);
        $crate::_bind_record_set_scan!(aiRecord, AiRecord, rsbind_ai_get_ioint_info);
        $crate::_bind_record_read!(aiRecord, AiRecord, rsbind_ai_read_ai, 2);
        $crate::_bind_record_linconv!(aiRecord, AiRecord, rsbind_ai_special_linconv);

        // ao record
        $crate::_bind_record_init!($record_init, aoRecord, AoRecord, rsbind_ao_init_record, 2);
        $crate::_bind_record_set_scan!(aoRecord, AoRecord, rsbind_ao_get_ioint_info);
        $crate::_bind_record_write!(aoRecord, AoRecord, rsbind_ao_write_ao);
        $crate::_bind_record_linconv!(aoRecord, AoRecord, rsbind_ao_special_linconv);
        // bi record
        $crate::_bind_record_init!($record_init, biRecord, BiRecord, rsbind_bi_init_record);
        $crate::_bind_record_set_scan!(biRecord, BiRecord, rsbind_bi_get_ioint_info);
        $crate::_bind_record_read!(biRecord, BiRecord, rsbind_bi_read_bi);

        // bo record
        $crate::_bind_record_init!($record_init, boRecord, BoRecord, rsbind_bo_init_record);
        $crate::_bind_record_set_scan!(boRecord, BoRecord, rsbind_bo_get_ioint_info);
        $crate::_bind_record_write!(boRecord, BoRecord, rsbind_bo_write_bo);

        // longin record
        $crate::_bind_record_init!($record_init, longinRecord, LonginRecord, rsbind_longin_init_record);
        $crate::_bind_record_set_scan!(longinRecord, LonginRecord, rsbind_longin_get_ioint_info);
        $crate::_bind_record_read!(longinRecord, LonginRecord, rsbind_longin_read_longin);

        // longout record
        $crate::_bind_record_init!($record_init, longoutRecord, LongoutRecord, rsbind_longout_init_record);
        $crate::_bind_record_set_scan!(longoutRecord, LongoutRecord, rsbind_longout_get_ioint_info);
        $crate::_bind_record_write!(longoutRecord, LongoutRecord, rsbind_longout_write_longout);

        // stringin record
        $crate::_bind_record_init!($record_init, stringinRecord, StringinRecord, rsbind_stringin_init_record);
        $crate::_bind_record_set_scan!(stringinRecord, StringinRecord, rsbind_stringin_get_ioint_info);
        $crate::_bind_record_read!(stringinRecord, StringinRecord, rsbind_stringin_read_stringin);

        // stringout record
        $crate::_bind_record_init!($record_init, stringoutRecord, StringoutRecord, rsbind_stringout_init_record);
        $crate::_bind_record_set_scan!(stringoutRecord, StringoutRecord, rsbind_stringout_get_ioint_info);
        $crate::_bind_record_write!(stringoutRecord, StringoutRecord, rsbind_stringout_write_stringout);
    };
}
