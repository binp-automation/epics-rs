use epics_sys::{IOSCANPVT};

use crate::record::*;
//use crate::context::Context;

use crate::asyncio;


pub unsafe fn record_init<R, F>(raw: R::Raw, f: F)
where R: Record + FromRaw + Into<AnyRecord>, F: Fn(&mut AnyRecord) -> AnyHandlerBox {
    let mut rec = R::from_raw(raw).into();
    rec.init();
    //let mut ctx = Context::new();
    let hdl = f(&mut rec);
    assert_eq!(rec.rtype(), hdl.rtype());
    rec.try_set_handler(hdl).expect("Record and handler type mismatch");
}

pub unsafe fn record_set_scan<R>(
    _detach: bool, raw: R::Raw, ppvt: *mut IOSCANPVT
) where R: ScanRecord + FromRaw {
    let mut rec = R::from_raw(raw);
    let scan = rec.create_scan();
    *ppvt = *scan.as_raw();
    rec.set_scan(scan.clone());
    //let mut ctx = Context::new();
    rec.handler_set_scan(scan);
}
pub unsafe fn record_read<R>(raw: R::Raw)
where R: ReadRecord + FromRaw + Into<AnyReadRecord> {
    let mut rec = R::from_raw(raw);
    if !rec.pact() {
        //let mut ctx = Context::new();
        if !rec.handler_read() {
            rec.set_pact(true);
            asyncio::record_read(rec.into());
        }
    }
}

pub unsafe fn record_write<R>(raw: R::Raw)
where R: WriteRecord + FromRaw + Into<AnyWriteRecord> {
    let mut rec = R::from_raw(raw);
    if !rec.pact() {
        //let mut ctx = Context::new();
        if !rec.handler_write() {
            rec.set_pact(true);
            asyncio::record_write(rec.into());
        }
    }
}

#[macro_export]
macro_rules! _bind_record_init {
    ($init:path, $raw:ident, $rec:ident, $xfn:ident) => {
        #[no_mangle]
        extern fn $xfn(
            rec: *mut $crate::epics_sys::$raw,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<$crate::record::$rec, _>(rec, $init); }
            0
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
            unsafe { $crate::devsup::record_set_scan::<$rec>(detach != 0, rec, ppvt); }
            0
        }
    }
}
#[macro_export]
macro_rules! _bind_record_read {
    ($raw:ident, $rec:ident, $xfn:ident) => {
        #[no_mangle]
        extern fn $xfn(
            rec: *mut $crate::epics_sys::$raw,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_read::<$rec>(rec); }
            0
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
            unsafe { $crate::devsup::record_write::<$rec>(rec); }
            0
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
            unsafe { <$crate::record::LinconvRecord>::handler_linconv(
                &mut $rec::from_raw(rec), after as i32
            ); }
            0
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
            unsafe { $crate::asyncio::start_loop(); }
            $init(unsafe { &mut $crate::context::Context::new() });
        }

        // ai record
        $crate::_bind_record_init!($record_init, aiRecord, AiRecord, rsbind_ai_init_record);
        $crate::_bind_record_set_scan!(aiRecord, AiRecord, rsbind_ai_get_ioint_info);
        $crate::_bind_record_read!(aiRecord, AiRecord, rsbind_ai_read_ai);
        $crate::_bind_record_linconv!(aiRecord, AiRecord, rsbind_ai_special_linconv);

        // ao record
        $crate::_bind_record_init!($record_init, aoRecord, AoRecord, rsbind_ao_init_record);
        $crate::_bind_record_set_scan!(aoRecord, AoRecord, rsbind_ao_get_ioint_info);
        $crate::_bind_record_write!(aoRecord, AoRecord, rsbind_ao_write_ao);
        $crate::_bind_record_linconv!(aoRecord, AoRecord, rsbind_ao_special_linconv);

        /*
        #[no_mangle]
        extern fn rsbind_ai_read_ai(
            rec: *mut $crate::epics_sys::aiRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_read::<AiRecord, _>(rec, $record_read); }
            0
        }
        #[no_mangle]
        extern fn rsbind_ai_special_linconv(
            _rec: *mut $crate::epics_sys::aiRecord,
            _after: $crate::libc::c_int,
        ) -> $crate::libc::c_long {
            0
        }


        #[no_mangle]
        extern fn rsbind_ao_init_record(
            rec: *mut $crate::epics_sys::aoRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<AoRecord, _>(rec, $record_init); }
            0
        }

        #[no_mangle]
        extern fn rsbind_ao_write_ao(
            rec: *mut $crate::epics_sys::aoRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_write::<AoRecord, _>(rec, $record_write); }
            0
        }
        #[no_mangle]
        extern fn rsbind_ao_special_linconv(
            _rec: *mut $crate::epics_sys::aoRecord,
            _after: $crate::libc::c_int,
        ) -> $crate::libc::c_long {
            0
        }

        // bi record

        #[no_mangle]
        extern fn rsbind_bi_init_record(
            rec: *mut $crate::epics_sys::biRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<BiRecord, _>(rec, $record_init); }
            0
        }
        #[no_mangle]
        extern fn rsbind_bi_read_bi(
            rec: *mut $crate::epics_sys::biRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_read::<BiRecord, _>(rec, $record_read); }
            0
        }

        // bo record

        #[no_mangle]
        extern fn rsbind_bo_init_record(
            rec: *mut $crate::epics_sys::boRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<BoRecord, _>(rec, $record_init); }
            0
        }
        #[no_mangle]
        extern fn rsbind_bo_write_bo(
            rec: *mut $crate::epics_sys::boRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_write::<BoRecord, _>(rec, $record_write); }
            0
        }

        // longin record

        #[no_mangle]
        extern fn rsbind_longin_init_record(
            rec: *mut $crate::epics_sys::longinRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<LonginRecord, _>(rec, $record_init); }
            0
        }
        #[no_mangle]
        extern fn rsbind_longin_read_longin(
            rec: *mut $crate::epics_sys::longinRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_read::<LonginRecord, _>(rec, $record_read); }
            0
        }

        // longout record

        #[no_mangle]
        extern fn rsbind_longout_init_record(
            rec: *mut $crate::epics_sys::longoutRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<LongoutRecord, _>(rec, $record_init); }
            0
        }
        #[no_mangle]
        extern fn rsbind_longout_write_longout(
            rec: *mut $crate::epics_sys::longoutRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_write::<LongoutRecord, _>(rec, $record_write); }
            0
        }

        // stringin record

        #[no_mangle]
        extern fn rsbind_stringin_init_record(
            rec: *mut $crate::epics_sys::stringinRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<StringinRecord, _>(rec, $record_init); }
            0
        }
        #[no_mangle]
        extern fn rsbind_stringin_read_stringin(
            rec: *mut $crate::epics_sys::stringinRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_read::<StringinRecord, _>(rec, $record_read); }
            0
        }

        // stringout record

        #[no_mangle]
        extern fn rsbind_stringout_init_record(
            rec: *mut $crate::epics_sys::stringoutRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<StringoutRecord, _>(rec, $record_init); }
            0
        }
        #[no_mangle]
        extern fn rsbind_stringout_write_stringout(
            rec: *mut $crate::epics_sys::stringoutRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_write::<StringoutRecord, _>(rec, $record_write); }
            0
        }
        */
    };
}

#[cfg(test)]
mod test {
    use crate::{
        bind_device_support,
        register_command,
        record::*,
        context::*,
    };

    struct AiTest {}
    impl AiHandler for AiTest {
        fn linconv(&mut self, _rec: &mut AiRecord, _after: i32) {}
    }
    impl ReadHandler<AiRecord> for AiTest {
        fn read(&mut self, _rec: &mut AiRecord) -> bool { false }
        fn read_async(&mut self, _rec: &mut AiRecord) {}
    }
    impl ScanHandler<AiRecord> for AiTest {
        fn set_scan(&mut self, _rec: &mut AiRecord, _scan: Scan) {}
    }

    fn init(context: &mut Context) {
        println!("[devsup] init");
        register_command!(context, fn test_command(a: i32, b: f64, c: &str) {
            println!("[devsup] test_command({}, {}, {})", a, b, c);
        });
    }
    fn record_init(_record: &mut AnyRecord) -> AnyHandlerBox {
        ((Box::new(AiTest {}) as Box<dyn AiHandler + Send>)).into()
    }

    bind_device_support!(
        init,
        record_init,
    );
}
