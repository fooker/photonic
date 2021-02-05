use std::time::Duration;

use crate::input::Input;
use std::thread::JoinHandle;

pub struct Ticker {
    thread: JoinHandle<()>,
}

impl Ticker {
    pub fn new(duration: Duration) -> (Self, Input<()>) {
        let input = Input::new();

        let mut sink = input.sink();

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
