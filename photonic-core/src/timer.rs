use std::time::Duration;

use crate::core::{Dynamic, Scene};
use crate::input::{Input, Sink};

pub struct Ticker {
    duration: Duration,
    remaining: Duration,
    sink: Sink<()>,
}

impl Ticker {
    pub fn new(scene: &mut Scene, name: &str, duration: Duration) -> Input<()> {
        let (input, sink) = Input::new();

        let ticker = Ticker {
            duration,
            remaining: duration,
            sink,
        };

        scene.register(name, ticker);

        return input;
    }
}

impl Dynamic for Ticker {
    fn update(&mut self, duration: &Duration) {
        if self.remaining < *duration {
            self.remaining += self.duration - *duration;
            self.sink.send(());
        } else {
            self.remaining -= *duration;
        }
    }
}
