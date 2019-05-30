use crate::record::{
    Scan,
    ScanRecord, ReadRecord, WriteRecord,
};


/// Handler for scannable records
pub trait ScanHandler<R: ScanRecord> {
    /// Set scan handle for `I/O Intr` records.
    fn set_scan(&mut self, rec: &mut R, scan: Scan);
}

/// Handler for records that could be read
pub trait ReadHandler<R: ReadRecord> {
    /// Synchronous read request. *Should not block.*
    ///
    /// Returns:
    /// + true is done,
    /// + false if need to be performed asynchronously
    fn read(&mut self, rec: &mut R) -> bool;
    /// Asynchronous read request, *may block*.
    ///
    /// This operation is performed in separate thread
    /// from thread pool and then notifies the EPICS.
    fn read_async(&mut self, rec: &mut R);
}

/// Handler for records that could be written
pub trait WriteHandler<R: WriteRecord> {
    /// Synchronous write request. *Should not block.*
    ///
    /// Returns:
    /// + true is done,
    /// + false if need to be performed asynchronously
    fn write(&mut self, rec: &mut R) -> bool;
    /// Asynchronous write request, *may block*.
    ///
    /// This operation is performed in separate thread
    /// from thread pool and then notifies the EPICS.
    fn write_async(&mut self, rec: &mut R);
}
