use std::time::Duration;

use tokio::task::JoinHandle;

use crate::InputHandle;

pub struct Ticker(JoinHandle<()>);

impl Ticker {
    pub fn new(period: Duration, input: &InputHandle<()>) -> Self {
        let sink = input.sink();

        let handle = tokio::spawn(async move {
            let mut timer = tokio::time::interval(period);
            loop {
                timer.tick().await;
                sink.send(());
            }
        });

        return Self(handle);
    }
}

impl Drop for Ticker {
    fn drop(&mut self) {
        self.0.abort();
    }
}
