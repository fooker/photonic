use std::fmt::Display;
use std::time::Duration;

use crate::input::{Input, Poll};

use super::*;

pub struct BoundManual<T> {
    bounds: Bounds<T>,

    input: Input<T>,
    current: T,
}

impl<T> Value<T> for BoundManual<T>
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

pub struct UnboundManual<T> {
    input: Input<T>,
    current: T,
}

impl<T> Value<T> for UnboundManual<T>
    where T: Copy {
    fn get(&self) -> T {
        self.current
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        if let Poll::Ready(update) = self.input.poll() {
            self.current = update;
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct ManualDecl<T> {
    pub input: Input<T>,
}

impl<T> From<Input<T>> for ManualDecl<T> {
    fn from(input: Input<T>) -> Self {
        return Self { input };
    }
}

impl<T> BoundValueDecl<T> for ManualDecl<T>
    where T: Copy + PartialOrd + Display + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let current = bounds.min;

        return Ok(Box::new(BoundManual {
            bounds,
            input: self.input,
            current,
        }));
    }
}

impl<T> UnboundValueDecl<T> for ManualDecl<T>
    where T: Copy + Default + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error> {
        return Ok(Box::new(UnboundManual {
            input: self.input,
            current: T::default(),
        }));
    }
}
