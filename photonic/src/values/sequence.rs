use std::sync::mpsc;
use std::time::Duration;

use failure::Error;

use crate::math;
use crate::trigger::Timer;

use super::*;

pub struct Sequence<T> {
    values: Vec<T>,

    position: usize,

    auto_trigger: Timer, // TODO: Unify auto_trigger and event input?
}

impl<T> Value<T> for Sequence<T>
    where T: Copy {
    fn get(&self) -> T {
        self.values[self.position]
    }

    fn update(&mut self, duration: &Duration) -> Update<T> {
        if self.auto_trigger.update(duration) {
            self.position = (self.position + 1) % self.values.len();
            return Update::Changed(self.values[self.position]);
        } else {
            return Update::Idle;
        }
    }
}

pub struct SequenceDecl<T> {
    values: Vec<T>,

    auto_trigger: Timer,
}

impl<T> BoundValueDecl<T> for SequenceDecl<T>
    where T: Copy + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {

        // TODO: Check bounds

        return Ok(Box::new(Sequence {
            values: self.values.clone(),
            position: 0,
            auto_trigger: self.auto_trigger,
        }));
    }
}

impl<T> UnboundValueDecl<T> for SequenceDecl<T>
    where T: Copy + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error> {
        return Ok(Box::new(Sequence {
            values: self.values.clone(),
            position: 0,
            auto_trigger: self.auto_trigger,
        }));
    }
}
