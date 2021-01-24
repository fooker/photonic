use std::time::Duration;

use crate::core::{Scene};
use crate::input::{Input, Sink};
use std::thread::JoinHandle;

pub struct Ticker {
    thread: JoinHandle<()>,
}

impl Ticker {
    pub fn new(duration: Duration) -> (Self, Input<()>) {
        let (input, mut sink) = Input::new();

        let thread = std::thread::spawn(move || {
            loop {
                std::thread::sleep(duration);
                sink.send(());
            }
        });

        return (Ticker {
            thread,
        }, input);
    }
}
