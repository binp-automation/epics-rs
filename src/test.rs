use std::{
    ops::Deref,
    str::from_utf8,
};

use crate::{
    self as epics,
    bind_device_support,
    register_command,
    record::*,
    context::*,
};

fn name<R: Deref<Target=Record>>(r: &R) -> &str {
    from_utf8(r.name()).unwrap()
}

macro_rules! impl_handler {
    ($Handler:ident, $opt:ident, $Record:ident) => {
        impl Handler<$Record> for $Handler {
            fn into_any_box(self) -> AnyHandlerBox {
                AnyHandlerBox::$opt(Box::new(self))
            }
        }
    };
}

macro_rules! impl_scan_handler {
    ($Handler:ident, $Record:ident) => {
        impl ScanHandler<$Record> for $Handler {
            fn set_scan(&mut self, record: &mut $Record, _scan: Scan) -> epics::Result<()> {
                println!("[DEVSUP] {}.set_scan({})", stringify!($Record), name(record));
                Ok(())
            }
        }
    };
}

macro_rules! impl_read_handler {
    ($Handler:ident, $Record:ident) => {
        impl ReadHandler<$Record> for $Handler {
            fn read(&mut self, record: &mut $Record) -> epics::Result<bool> {
                println!("[DEVSUP] {}.read({})", stringify!($Record), name(record));
                Ok(false)
            }
            fn read_async(&mut self, record: &mut $Record) -> epics::Result<()> {
                println!("[DEVSUP] {}.read_async({})", stringify!($Record), name(record));
                Ok(())
            }
        }
    };
}

macro_rules! impl_write_handler {
    ($Handler:ident, $Record:ident) => {
        impl WriteHandler<$Record> for $Handler {
            fn write(&mut self, record: &mut $Record) -> epics::Result<bool> {
                println!("[DEVSUP] {}.write({})", stringify!($Record), name(record));
                Ok(false)
            }
            fn write_async(&mut self, record: &mut $Record) -> epics::Result<()> {
                println!("[DEVSUP] {}.write_async({})", stringify!($Record), name(record));
                Ok(())
            }
        }
    };
}

struct AiTest {}
impl_handler!(AiTest, Ai, AiRecord);
impl_scan_handler!(AiTest, AiRecord);
impl_read_handler!(AiTest, AiRecord);
impl AiHandler for AiTest {}

struct AoTest {}
impl_handler!(AoTest, Ao, AoRecord);
impl_scan_handler!(AoTest, AoRecord);
impl_write_handler!(AoTest, AoRecord);
impl AoHandler for AoTest {}

struct BiTest {}
impl_handler!(BiTest, Bi, BiRecord);
impl_scan_handler!(BiTest, BiRecord);
impl_read_handler!(BiTest, BiRecord);
impl BiHandler for BiTest {}

struct BoTest {}
impl_handler!(BoTest, Bo, BoRecord);
impl_scan_handler!(BoTest, BoRecord);
impl_write_handler!(BoTest, BoRecord);
impl BoHandler for BoTest {}

struct LonginTest {}
impl_handler!(LonginTest, Longin, LonginRecord);
impl_scan_handler!(LonginTest, LonginRecord);
impl_read_handler!(LonginTest, LonginRecord);
impl LonginHandler for LonginTest {}

struct LongoutTest {}
impl_handler!(LongoutTest, Longout, LongoutRecord);
impl_scan_handler!(LongoutTest, LongoutRecord);
impl_write_handler!(LongoutTest, LongoutRecord);
impl LongoutHandler for LongoutTest {}

struct StringinTest {}
impl_handler!(StringinTest, Stringin, StringinRecord);
impl_scan_handler!(StringinTest, StringinRecord);
impl_read_handler!(StringinTest, StringinRecord);
impl StringinHandler for StringinTest {}

struct StringoutTest {}
impl_handler!(StringoutTest, Stringout, StringoutRecord);
impl_scan_handler!(StringoutTest, StringoutRecord);
impl_write_handler!(StringoutTest, StringoutRecord);
impl StringoutHandler for StringoutTest {}


fn init(context: &mut Context) -> epics::Result<()> {
    println!("[DEVSUP] init");
    register_command!(context, fn test_command(a: i32, b: f64, c: &str) -> epics::Result<()> {
        println!("[DEVSUP] test_command({}, {}, {})", a, b, c);
        Ok(())
    });
    Ok(())
}
fn record_init(record: &mut AnyRecord) -> epics::Result<AnyHandlerBox> {
    println!("[DEVSUP] record_init {:?}: {}", record.rtype(), name(record));
    Ok(match record {
        AnyRecord::Ai(_) => (AiTest {}).into_any_box(),
        AnyRecord::Ao(_) => (AoTest {}).into_any_box(),
        AnyRecord::Bi(_) => (BiTest {}).into_any_box(),
        AnyRecord::Bo(_) => (BoTest {}).into_any_box(),
        AnyRecord::Longin(_) => (LonginTest {}).into_any_box(),
        AnyRecord::Longout(_) => (LongoutTest {}).into_any_box(),
        AnyRecord::Stringin(_) => (StringinTest {}).into_any_box(),
        AnyRecord::Stringout(_) => (StringoutTest {}).into_any_box(),
    })
}

bind_device_support!(
    init,
    record_init,
);
