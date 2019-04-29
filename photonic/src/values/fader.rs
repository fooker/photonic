use std::time::Duration;

use super::*;
use crate::input::{Poll, Input};
use std::fmt::Display;

pub struct Fader<T> {
    bounds: Bounds<T>,

    input: Input<T>,
    current: T,
}

impl<T> Value<T> for Fader<T>
    where T: Copy + PartialOrd + Display {
    fn get(&self) -> T {
        self.current
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        if let Poll::Ready(update) = self.input.poll() {
            if let Ok(update) = self.bounds.ensure(update) {
                self.current = update;
                return Update::Changed(self.current);
            } else {
                return Update::Idle;
            }
        } else {
            return Update::Idle;
        }
    }
}

pub struct FaderDecl<T> {
    input: Input<T>,
}

impl<T> BoundValueDecl<T> for FaderDecl<T>
    where T: Copy + PartialOrd + Display + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let current = bounds.min;

        return Ok(Box::new(Fader {
            bounds,
            input: self.input,
            current,
        }));
    }
}
