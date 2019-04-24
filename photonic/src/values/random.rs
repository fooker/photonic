use std::sync::mpsc;
use std::time::Duration;

use failure::Error;
use rand::distributions::uniform::SampleUniform;
use rand::prelude::{FromEntropy, Rng, SmallRng};

use crate::math;
use crate::trigger::Timer;

use super::*;

pub struct Random<T> {
    min: T,
    max: T,

    current: T,

    random: SmallRng,

    auto_trigger: Timer, // TODO: Unify auto_trigger and event input?
//    update: (mpsc::SyncSender<()>, mpsc::Receiver<()>),
}

impl<T> Value<T> for Random<T>
    where T: Copy + SampleUniform {
    fn get(&self) -> T {
        self.current
    }

    fn update(&mut self, duration: &Duration) -> Update<T> {
        if self.auto_trigger.update(duration) {
            self.current = self.random.gen_range(self.min, self.max);
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct RandomDecl {
    pub auto_trigger: Timer,
}

impl<T> BoundValueDecl<T> for RandomDecl
    where T: Copy + SampleUniform + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let (min, max) = bounds.into();

        return Ok(Box::new(Random {
            min,
            max,
            current: min, // TODO: Start with a random value?
            random: SmallRng::from_entropy(),
            auto_trigger: self.auto_trigger,
        }));
    }
}
