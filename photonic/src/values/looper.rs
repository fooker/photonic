use std::sync::mpsc;
use std::time::Duration;

use failure::Error;
use num::traits::Num;

use crate::math;
use crate::trigger::Timer;

use super::*;

pub struct Looper<T>
    where T: Num {
    min: T,
    max: T,
    step: T,

    current: T,

    auto_trigger: Timer, // TODO: Unify auto_trigger and event input?
//    update: (mpsc::SyncSender<()>, mpsc::Receiver<()>),
}

impl<T> Value<T> for Looper<T>
    where T: Copy + Num {

    fn get(&self) -> T {
        return self.current;
    }

    fn update(&mut self, duration: &Duration) -> Update<T> {
        if self.auto_trigger.update(duration) {
            self.current = self.min + (self.current + self.step - self.min) % (self.max - self.min);
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct LooperDecl<T> {
    pub step: T,

    pub auto_trigger: Timer,
}

impl<T> BoundValueDecl<T> for LooperDecl<T>
    where T: Copy + PartialOrd + Num + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let (min, max) = bounds.into();

        let step = if self.step >= T::zero() { self.step } else {
            (self.step % (max - min + T::one())) + (max - min + T::one())
        };

        let max = max + T::one();

        return Ok(Box::new(Looper {
            min,
            max,
            step,
            current: min,
            auto_trigger: self.auto_trigger,
//            update: mpsc::sync_channel(0),
        }));
    }
}
