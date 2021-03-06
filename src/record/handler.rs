use crate::record::{
    Scan, Record,
    ScanRecord, ReadRecord, WriteRecord,
};

/// Base of all handlers
pub trait Handler<R: Record> {}

/// Handler that able to be initialized
pub trait InitHandler<R: Record>: Handler<R> {
    /// Called on record initialization.
    /// Takes record itself and arguments from INP or OUT field.
    ///
    /// Returns new handler instance.
    fn init(rec: &mut R, args: &[&str]) -> crate::Result<Self> where Self: Sized;
}

/// Handler for scannable records
pub trait ScanHandler<R: ScanRecord>: Handler<R> {
    /// Set scan handle for `I/O Intr` records.
    fn set_scan(&mut self, rec: &mut R, scan: Scan) -> crate::Result<()>;
}

/// Handler for records that could be read
pub trait ReadHandler<R: ReadRecord>: Handler<R> {
    /// Synchronous read request. *Should not block.*
    ///
    /// Returns:
    /// + true is done,
    /// + false if need to be performed asynchronously
    fn read(&mut self, rec: &mut R) -> crate::Result<bool>;
    /// Asynchronous read request, *may block*.
    ///
    /// This operation is performed in separate thread
    /// from thread pool and then notifies the EPICS.
    fn read_async(&mut self, rec: &mut R) -> crate::Result<()>;
}

/// Handler for records that could be written
pub trait WriteHandler<R: WriteRecord>: Handler<R> {
    /// Synchronous write request. *Should not block.*
    ///
    /// Returns:
    /// + true is done,
    /// + false if need to be performed asynchronously
    fn write(&mut self, rec: &mut R) -> crate::Result<bool>;
    /// Asynchronous write request, *may block*.
    ///
    /// This operation is performed in separate thread
    /// from thread pool and then notifies the EPICS.
    fn write_async(&mut self, rec: &mut R) -> crate::Result<()>;
}
