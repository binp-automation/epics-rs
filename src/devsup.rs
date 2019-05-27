//use epics_sys::{dbCommon, IOSCANPVT};

use crate::record::*;
use crate::context::Context;

//use crate::asyncio;


pub unsafe fn record_init<'a, R: 'a, F>(raw: *mut R, f: F)
where R: Record, &'a mut R: Into<AnyRecordRef<'a>>, F: Fn(&mut Context, AnyRecordRef) {
    let rec = raw.as_mut().unwrap();
    rec.init();
    let mut ctx = Context::new();
    f(&mut ctx, rec.into());
    // TODO: set private
}
/*
pub unsafe fn record_set_scan<R, F>(
    _detach: bool,
    raw: *mut R,
    ppvt: *mut IOSCANPVT,
    f: F,
) where R: Record, F: Fn(&mut Context, &mut RecordBase, Scan) {
    let mut rec = RecordBase::from_raw(raw);
    let scan = rec.get_scan().clone();
    *ppvt = *scan.as_raw();
    let mut ctx = Context::new();
    f(&mut ctx, &mut rec, scan.clone());
}

pub unsafe fn record_read<R, F>(raw: R::Raw, f: F)
where R: Record + Into<ReadRecord>, F: Fn(&mut Context, &mut ReadRecord) {
    let mut rec = R::from_raw(raw).into();
    if !rec.pact() {
        let mut ctx = Context::new();
        f(&mut ctx, &mut rec);
        if rec.pact() {
            asyncio::record_read(rec);
        }
    }
}

pub unsafe fn record_write<R, F>(raw: R::Raw, f: F)
where R: Record + Into<WriteRecord>, F: Fn(&mut Context, &mut WriteRecord) {
    let mut rec = R::from_raw(raw).into();
    if !rec.pact() {
        let mut ctx = Context::new();
        f(&mut ctx, &mut rec);
        if rec.pact() {
            asyncio::record_write(rec);
        }
    }
}
*/
#[macro_export]
macro_rules! bind_device_support {
    ( $( $x:path ),* ) => {
        $crate::bind_device_support!( $( $x, )* );
    };
    (
        $init:path,
        $quit:path,
        $record_init:path,
        $record_set_scan:path,
        $record_read:path,
        $record_write:path,
    ) => {
        $crate::_bind_device_support_init!(
            $init,
            $quit,
        );
        $crate::_bind_device_support_record!(
            $record_init,
            $record_set_scan,
            $record_read,
            $record_write,
        );
    };
    (
        $init:path,
        $quit:path,
        $record_init:path,
        $record_set_scan:path,
        $record_read:path,
        $record_write:path,
        $record_read_async:path,
        $record_write_async:path,
    ) => {
        $crate::_bind_device_support_init!(
            $init,
            $quit,
            $record_read_async,
            $record_write_async,
        );
        $crate::_bind_device_support_record!(
            $record_init,
            $record_set_scan,
            $record_read,
            $record_write,
        );
    }
}

#[macro_export]
macro_rules! _bind_device_support_init {
    (
        $init:path,
        $quit:path,
    ) => {
        #[no_mangle]
        extern fn rsbind_init() {
            $init(unsafe { &mut $crate::context::Context::new() });
        }

        #[no_mangle]
        extern fn rsbind_quit() {
            $quit(unsafe { &mut $crate::context::Context::new() });
        }
    };
    (
        $init:path,
        $quit:path,
        $record_read_async:path,
        $record_write_async:path,
    ) => {
        #[no_mangle]
        extern fn rsbind_init() {
            unsafe { $crate::asyncio::start_loop(
                $record_read_async,
                $record_write_async,
            ); }
            $init(unsafe { &mut $crate::context::Context::new() });
        }

        #[no_mangle]
        extern fn rsbind_quit() {
            $quit(unsafe { &mut $crate::context::Context::new() });
            unsafe { $crate::asyncio::stop_loop(); }
        }
    }
}

#[macro_export]
macro_rules! _bind_device_support_record {
    (
        $record_init:path,
        $record_set_scan:path,
        $record_read:path,
        $record_write:path,
    ) => {
        // any record

        #[no_mangle]
        extern fn rsbind_get_ioint_info(
            detach: $crate::libc::c_int,
            rec: *mut $crate::epics_sys::dbCommon,
            ppvt: *mut $crate::epics_sys::IOSCANPVT,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_set_scan(detach != 0, rec, ppvt, $record_set_scan); }
            0
        }

        // ai record

        #[no_mangle]
        extern fn rsbind_ai_init_record(
            rec: *mut $crate::epics_sys::aiRecord,
        ) -> $crate::libc::c_long {
            unsafe { $crate::devsup::record_init::<AiRecord, _>(rec, $record_init); }
            0
        }
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

        // ao record

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

        /*
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

/*
#[cfg(test)]
mod test {
    use std::str::from_utf8;

    use crate::{
        bind_device_support,
        register_command,
        record::*,
        context::*,
    };


    fn init(context: &mut Context) {
        println!("[devsup] init");
        register_command!(context, fn test_command(a: i32, b: f64, c: &str) {
            println!("[devsup] test_command({}, {}, {})", a, b, c);
        });
    }
    fn quit(_context: &mut Context) {
        println!("[devsup] quit");
    }
    fn record_init(_context: &mut Context, record: &mut AnyRecord) {
        println!("[devsup] record_init {}", from_utf8(record.name()).unwrap());
    }
    fn record_set_scan(_context: &mut Context, record: &mut RecordBase, _scan: Scan) {
        println!("[devsup] record_set_scan {}", from_utf8(record.name()).unwrap());
    }
    fn record_read(context: &mut Context, record: &mut ReadRecord) {
        println!("[devsup] record_read {}", from_utf8(record.name()).unwrap());
        context.request_async(record);
    }
    fn record_write(context: &mut Context, record: &mut WriteRecord) {
        println!("[devsup] record_write {}", from_utf8(record.name()).unwrap());
        context.request_async(record);
    }
    fn record_read_async(_context: &mut Context, record: &mut ReadRecord) {
        println!("[devsup] record_read_async {}", from_utf8(record.name()).unwrap());
    }
    fn record_write_async(_context: &mut Context, record: &mut WriteRecord) {
        println!("[devsup] record_write_async {}", from_utf8(record.name()).unwrap());
    }

    bind_device_support!(
        init,
        quit,
        record_init,
        record_set_scan,
        record_read,
        record_write,
        record_read_async,
        record_write_async,
    );
}
*/
