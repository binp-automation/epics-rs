use std::io;

use crate::record::{Scan, Record, AnyRecord, ReadRecord, WriteRecord};

pub trait DeviceSupport {
    fn init(&mut self, record: &mut AnyRecord) -> io::Result<()>;
    fn read(&mut self, record: &mut ReadRecord) -> io::Result<()>;
    fn write(&mut self, record: &mut WriteRecord) -> io::Result<()>;
    fn set_scan(&mut self, record: &mut Record, scan: Scan) -> io::Result<()>;
}
