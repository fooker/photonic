use crate::values::{DynamicValue, Update};
use std::time::Duration;
use rand::prelude::{SmallRng,Rng,FromEntropy};
use crate::trigger::Timer;
use crate::animation::Easing;
use crate::math;
use std::sync::mpsc;

pub struct Random {
    min: f64,
    max: f64,

    current: f64,

    random: SmallRng,

    auto_trigger: Timer, // TODO: Unify auto_trigger and event input?
    update: (mpsc::SyncSender<()>, mpsc::Receiver<()>),
}

impl Random {
    pub fn new(min: f64, max: f64,
               auto_trigger: Option<Duration>) -> Self {
        let (min, max) = math::minmax(min, max);

        return Self {
            min,
            max,
            current: min,
            random: SmallRng::from_entropy(),
            auto_trigger: Timer::new(auto_trigger),
            update: mpsc::sync_channel(0),
        };
    }

    pub fn updater(&self) -> mpsc::SyncSender<()> {
        // FIXME: Return setter lambda function
        return self.update.0.clone();
    }
}

impl DynamicValue<f64> for Random {
    fn get(&self) -> f64 {
        self.current
    }

    fn update(&mut self, duration: &Duration) -> Update<f64> {
        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            self.current = self.random.gen_range(self.min, self.max);
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}
