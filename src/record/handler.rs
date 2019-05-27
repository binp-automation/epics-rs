pub trait Handler {
	fn set_scan(&mut self);
}

pub trait ReadHandler {
	fn read(&mut self);
	fn read_async(&mut self);
}
pub trait WriteHandler {
	fn write(&mut self);
	fn write_async(&mut self);
}

pub trait AiHandler: Handler + ReadHandler {}
pub trait AoHandler: Handler + WriteHandler {}
