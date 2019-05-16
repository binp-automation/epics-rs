/*
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Mutex};
use driver;

use crate::action::{Action, AsyncAction};


enum Message {
    Break,
    Action(AsyncAction)
}

struct Handler {
    channel: Sender<Message>,
    thread: Mutex<JoinHandle<()>>,
}

static handler: Option<Handler> = None;

fn handler_loop(channel: Receiver<Message>) {
    loop {
        match channel.recv().unwrap() {
            Message::Break => break,
            _ => (),
        }
    }
}
*/
pub(crate) fn init() {
    //let (tx, rx) = mpsc::channel();
    //let jh = thread::spawn(move || handler_loop(rx));
}

pub(crate) fn quit() {
    
}