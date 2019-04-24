use crate::values::{DynamicValue, Update};
use std::time::Duration;
use crate::trigger::Timer;
use crate::animation::Easing;
use std::sync::mpsc;

pub struct Sequence {
    values: Vec<f64>,

    position: usize,

    auto_trigger: Timer, // TODO: Unify auto_trigger and event input?
    update: (mpsc::SyncSender<()>, mpsc::Receiver<()>),
}

impl Sequence {
    // TODO: Allow manual switching
    pub fn new(values: Vec<f64>,
               auto_trigger: Option<Duration>) -> Self {
        Self {
            values: values.clone(),
            position: 0,
            auto_trigger: Timer::new(auto_trigger),
            update: mpsc::sync_channel(0),
        }
    }

    pub fn updater(&self) -> mpsc::SyncSender<()> {
        // FIXME: Return setter lambda function
        return self.update.0.clone();
    }
}

impl DynamicValue<f64> for Sequence {
    fn get(&self) -> f64 {
        return self.values[self.position];
    }

    fn update(&mut self, duration: &Duration) -> Update<f64> {
        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            self.position = (self.position + 1) % self.values.len();
            return Update::Changed(self.values[self.position]);
        } else {
            return Update::Idle;
        }
    }
}
