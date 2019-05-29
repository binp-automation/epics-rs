use std::{
    ops::Deref,
    str::from_utf8,
};

use crate::{
    bind_device_support,
    register_command,
    record::*,
    context::*,
};

fn name<R: Deref<Target=Record>>(r: &R) -> &str {
    from_utf8(r.name()).unwrap()
}

macro_rules! impl_scan_handler {
    ($Handler:ident, $Record:ident) => {
        impl ScanHandler<$Record> for $Handler {
            fn set_scan(&mut self, record: &mut $Record, _scan: Scan) {
                println!("[devsup] {}.set_scan({})", stringify!($Record), name(record));
            }
        }
    };
}

macro_rules! impl_read_handler {
    ($Handler:ident, $Record:ident) => {
        impl ReadHandler<$Record> for $Handler {
            fn read(&mut self, record: &mut $Record) -> bool {
                println!("[devsup] {}.read({})", stringify!($Record), name(record));
                false
            }
            fn read_async(&mut self, record: &mut $Record) {
                println!("[devsup] {}.read_async({})", stringify!($Record), name(record));
            }
        }
    };
}

macro_rules! impl_write_handler {
    ($Handler:ident, $Record:ident) => {
        impl WriteHandler<$Record> for $Handler {
            fn write(&mut self, record: &mut $Record) -> bool {
                println!("[devsup] {}.write({})", stringify!($Record), name(record));
                false
            }
            fn write_async(&mut self, record: &mut $Record) {
                println!("[devsup] {}.write_async({})", stringify!($Record), name(record));
            }
        }
    };
}

struct AiTest {}
impl_scan_handler!(AiTest, AiRecord);
impl_read_handler!(AiTest, AiRecord);
impl AiHandler for AiTest {
    fn linconv(&mut self, record: &mut AiRecord, _after: i32) {
        println!("[devsup] AiRecord.linconv({})", name(record));
    }
}

struct AoTest {}
impl_scan_handler!(AoTest, AoRecord);
impl_write_handler!(AoTest, AoRecord);
impl AoHandler for AoTest {
    fn linconv(&mut self, record: &mut AoRecord, _after: i32) {
        println!("[devsup] AoRecord.linconv({})", name(record));
    }
}

struct BiTest {}
impl_scan_handler!(BiTest, BiRecord);
impl_read_handler!(BiTest, BiRecord);
impl BiHandler for BiTest {}

struct BoTest {}
impl_scan_handler!(BoTest, BoRecord);
impl_write_handler!(BoTest, BoRecord);
impl BoHandler for BoTest {}

struct LonginTest {}
impl_scan_handler!(LonginTest, LonginRecord);
impl_read_handler!(LonginTest, LonginRecord);
impl LonginHandler for LonginTest {}

struct LongoutTest {}
impl_scan_handler!(LongoutTest, LongoutRecord);
impl_write_handler!(LongoutTest, LongoutRecord);
impl LongoutHandler for LongoutTest {}

struct StringinTest {}
impl_scan_handler!(StringinTest, StringinRecord);
impl_read_handler!(StringinTest, StringinRecord);
impl StringinHandler for StringinTest {}

struct StringoutTest {}
impl_scan_handler!(StringoutTest, StringoutRecord);
impl_write_handler!(StringoutTest, StringoutRecord);
impl StringoutHandler for StringoutTest {}


fn init(context: &mut Context) {
    println!("[devsup] init");
    register_command!(context, fn test_command(a: i32, b: f64, c: &str) {
        println!("[devsup] test_command({}, {}, {})", a, b, c);
    });
}
fn record_init(record: &mut AnyRecord) -> AnyHandlerBox {
    println!("[devsup] record_init {:?}: {}", record.rtype(), name(record));
    match record {
        AnyRecord::Ai(_) => ((Box::new(AiTest {}) as Box<dyn AiHandler + Send>)).into(),
        AnyRecord::Ao(_) => ((Box::new(AoTest {}) as Box<dyn AoHandler + Send>)).into(),
        AnyRecord::Bi(_) => ((Box::new(BiTest {}) as Box<dyn BiHandler + Send>)).into(),
        AnyRecord::Bo(_) => ((Box::new(BoTest {}) as Box<dyn BoHandler + Send>)).into(),
        AnyRecord::Longin(_) => ((Box::new(LonginTest {}) as Box<dyn LonginHandler + Send>)).into(),
        AnyRecord::Longout(_) => ((Box::new(LongoutTest {}) as Box<dyn LongoutHandler + Send>)).into(),
        AnyRecord::Stringin(_) => ((Box::new(StringinTest {}) as Box<dyn StringinHandler + Send>)).into(),
        AnyRecord::Stringout(_) => ((Box::new(StringoutTest {}) as Box<dyn StringoutHandler + Send>)).into(),
    }
}

bind_device_support!(
    init,
    record_init,
);
