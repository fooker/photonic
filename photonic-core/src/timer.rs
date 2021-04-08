use std::thread::JoinHandle;
use std::time::Duration;

use crate::input::Input;

pub struct Ticker {
    _thread: JoinHandle<()>,
}

impl Ticker {
    pub fn new(duration: Duration) -> (Self, Input<()>) {
        let input = Input::new();

        let sink = input.sink();

        let thread = std::thread::spawn(move || loop {
            std::thread::sleep(duration);
            sink.send(());
        });

        return (Ticker { _thread: thread }, input);
    }
}
