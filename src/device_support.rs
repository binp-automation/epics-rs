use epics_sys::{IOSCANPVT};

use crate::record::*;
//use crate::context::Context;

use crate::async_proc;


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
            async_proc::record_read(rec.into());
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
            async_proc::record_write(rec.into());
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
            unsafe { $crate::device_support::record_init::<$crate::record::$rec, _>(rec, $init); }
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
            unsafe { $crate::device_support::record_set_scan::<$rec>(detach != 0, rec, ppvt); }
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
            unsafe { $crate::device_support::record_read::<$rec>(rec); }
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
            unsafe { $crate::device_support::record_write::<$rec>(rec); }
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
            unsafe { $crate::async_proc::start_loop(); }
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
