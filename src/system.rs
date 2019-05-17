use std::cell::Cell;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::record::{Scan, Record, AnyRecord, ReadRecord, WriteRecord};
use crate::devsup::{DeviceSupport};

pub(crate) enum Message {
    Break,
    Read(ReadRecord),
    Write(WriteRecord),
}

struct Handler {
    devsup: Box<DeviceSupport + Send>,
    channel: Sender<Message>,
    thread: JoinHandle<()>,
}


lazy_static! {
    static ref HANDLER: Mutex<Option<Handler>> = Mutex::new(None);
}

thread_local! {
    static CHANNEL: Cell<Option<Sender<Message>>> = Cell::new(None);
}

fn with_devsup<F: FnOnce(&mut DeviceSupport)>(f: F) {
    let mut guard = HANDLER.lock().unwrap();
    f(guard.as_mut().unwrap().devsup.as_mut());
}

fn handler_loop(channel: Receiver<Message>) {
    loop {
        match channel.recv().unwrap() {
            Message::Break => break,
            Message::Read(mut rec) => {
                with_devsup(|ds| ds.read(&mut rec).unwrap());
                rec.process();
            },
            Message::Write(mut rec) => {
                with_devsup(|ds| ds.write(&mut rec).unwrap());
                rec.process();
            },
        }
    }
}

pub fn init(devsup: Box<DeviceSupport + Send>) {
    println!("[rsbind] init");
    let (tx, rx) = mpsc::channel();
    let jh = thread::spawn(move || handler_loop(rx));
    let mut guard = HANDLER.lock().unwrap();
    *guard = Some(Handler { devsup, channel: tx, thread: jh })
}

pub(crate) fn quit() {
    println!("[rsbind] quit");
    let handler = {
        let mut guard = HANDLER.lock().unwrap();
        guard.take().unwrap()
    };
    handler.channel.send(Message::Break).unwrap();
    handler.thread.join().unwrap()
}

fn with_channel<F: FnOnce(&Sender<Message>)>(f: F) {
    CHANNEL.with(|chan_cell| {
        let channel = match chan_cell.replace(None) {
            Some(chan) => chan,
            None => (*HANDLER.lock().unwrap()).as_ref().unwrap().channel.clone(),
        };
        f(&channel);
        assert!(chan_cell.replace(Some(channel)).is_none())
    });
}

pub(crate) fn record_init(mut record: AnyRecord) {
    println!("[rsbind] record_init({})", record.name());
    with_devsup(|ds| ds.init(&mut record).unwrap());
}

pub(crate) fn record_ioint(_detach: bool, mut record: Record) -> Scan {
    println!("[rsbind] record_ioint({})", record.name());
    let scan = record.scan().clone();
    with_devsup(|ds| ds.set_scan(&mut record, scan.clone()).unwrap());
    scan
}

pub(crate) fn record_write(mut record: WriteRecord) {
    println!("[rsbind] record_write({})", record.name());
    if !record.pact() {
        record.set_pact(true);
        with_channel(|channel| {
            channel.send(Message::Write(record)).unwrap();
        });
    }
}

pub(crate) fn record_read(mut record: ReadRecord) {
    println!("[rsbind] record_read({})", record.name());
    if !record.pact() {
        record.set_pact(true);
        with_channel(|channel| {
            channel.send(Message::Read(record)).unwrap();
        });
    }
}
