use simple_logger;

use crate::{
    self as epics,
    log::{info},
    bind_device_support,
    register_command,
    record::*,
    context::*,
};

macro_rules! impl_handler {
    ($Handler:ident, $opt:ident, $Record:ident) => {
        impl Handler<$Record> for $Handler {}
        impl InitHandler<$Record> for $Handler {
            fn init(_record: &mut $Record, _args: &[&str]) -> epics::Result<Self> {
                Ok(Self {})
            }
        }
    };
}

macro_rules! impl_scan_handler {
    ($Handler:ident, $Record:ident) => {
        impl ScanHandler<$Record> for $Handler {
            fn set_scan(&mut self, record: &mut $Record, _scan: Scan) -> epics::Result<()> {
                info!("{}.set_scan({})", stringify!($Record), record.name());
                Ok(())
            }
        }
    };
}

macro_rules! impl_read_handler {
    ($Handler:ident, $Record:ident) => {
        impl ReadHandler<$Record> for $Handler {
            fn read(&mut self, record: &mut $Record) -> epics::Result<bool> {
                info!("{}.read({})", stringify!($Record), record.name());
                Ok(false)
            }
            fn read_async(&mut self, record: &mut $Record) -> epics::Result<()> {
                info!("{}.read_async({})", stringify!($Record), record.name());
                Ok(())
            }
        }
    };
}

macro_rules! impl_write_handler {
    ($Handler:ident, $Record:ident) => {
        impl WriteHandler<$Record> for $Handler {
            fn write(&mut self, record: &mut $Record) -> epics::Result<bool> {
                info!("{}.write({})", stringify!($Record), record.name());
                Ok(false)
            }
            fn write_async(&mut self, record: &mut $Record) -> epics::Result<()> {
                info!("{}.write_async({})", stringify!($Record), record.name());
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
    simple_logger::init().unwrap();
    info!("init");
    register_command!(context, fn test_command(a: i32, b: f64, c: &str) -> epics::Result<()> {
        info!("test_command({}, {}, {})", a, b, c);
        Ok(())
    });
    Ok(())
}

bind_device_support!(
    init,
    {
        AiTest,
        AoTest,
        BiTest,
        BoTest,
        LonginTest,
        LongoutTest,
        StringinTest,
        StringoutTest,
    },
);
