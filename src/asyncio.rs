use std::cell::Cell;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::record::{ReadRecord, WriteRecord};

pub(crate) enum Message {
    Break,
    Read(ReadRecord),
    Write(WriteRecord),
}

struct Handler {
    channel: Sender<Message>,
    thread: JoinHandle<()>,
}

lazy_static! {
    static ref HANDLER: Mutex<Option<Handler>> = Mutex::new(None);
}

thread_local! {
    static CHANNEL: Cell<Option<Sender<Message>>> = Cell::new(None);
}

fn handler_loop<FR, FW>(channel: Receiver<Message>, fr: FR, fw: FW)
where FR: Fn(&mut ReadRecord),
      FW: Fn(&mut WriteRecord) {
    loop {
        match channel.recv().unwrap() {
            Message::Break => break,
            Message::Read(mut rec) => {
                fr(&mut rec);
                unsafe { rec.process() };
            },
            Message::Write(mut rec) => {
                fw(&mut rec);
                unsafe { rec.process() };
            },
        }
    }
}

pub unsafe fn start_loop<FR, FW>(fr: FR, fw: FW)
where FR: 'static + Fn(&mut ReadRecord) + Send,
      FW: 'static + Fn(&mut WriteRecord) + Send {
    let (tx, rx) = mpsc::channel();
    let jh = thread::spawn(move || handler_loop(rx, fr, fw));
    let mut guard = HANDLER.lock().unwrap();
    assert!(guard.is_none());
    *guard = Some(Handler { channel: tx, thread: jh })
}

pub unsafe fn stop_loop() {
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

pub unsafe fn record_write(record: WriteRecord) {
    with_channel(|channel| {
        channel.send(Message::Write(record)).unwrap();
    });
}

pub unsafe fn record_read(record: ReadRecord) {
    with_channel(|channel| {
        channel.send(Message::Read(record)).unwrap();
    });
}
