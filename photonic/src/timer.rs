use std::cell::RefCell;
use std::time::Duration;

use crate::input::{Input, Sink};

pub struct Timer {
    tickers: RefCell<Vec<Ticker>>,
}

impl Timer {
    pub fn new() -> Self {
        return Self {
            tickers: RefCell::new(Vec::new()),
        };
    }

    pub fn ticker(&mut self, duration: Duration) -> Input<()> {
        let (input, sink) = Input::new();

        let ticker = Ticker {
            duration,
            remaining: duration,
            sink,
        };

        self.tickers.borrow_mut().push(ticker);

        return input;
    }

    pub fn update(&mut self, duration: &Duration) {
        for ticker in self.tickers.borrow_mut().iter_mut() {
            ticker.update(duration);
        }
    }
}

struct Ticker {
    duration: Duration,
    remaining: Duration,
    sink: Sink<()>,
}

impl Ticker {
    pub(self) fn update(&mut self, duration: &Duration) {
        if self.remaining < *duration {
            self.remaining += self.duration - *duration;
            self.sink.send(());
        } else {
            self.remaining -= *duration;
        }
    }
}
