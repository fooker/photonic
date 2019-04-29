use std::time::Duration;

use failure::Error;

use crate::input::{Input, Poll};

use super::*;
use std::fmt::Display;

pub struct Sequence<T> {
    values: Vec<T>,

    position: usize,

    trigger: Input<()>,
}

impl<T> Value<T> for Sequence<T>
    where T: Copy {
    fn get(&self) -> T {
        self.values[self.position]
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        if let Poll::Ready(()) = self.trigger.poll() {
            self.position = (self.position + 1) % self.values.len();
            return Update::Changed(self.values[self.position]);
        } else {
            return Update::Idle;
        }
    }
}

pub struct SequenceDecl<T> {
    pub values: Vec<T>,
    pub trigger: Input<()>,
}

impl<T> BoundValueDecl<T> for SequenceDecl<T>
    where T: Copy + PartialOrd + Display + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let values = self.values.into_iter()
            .map(|v| bounds.ensure(v))
            .collect::<Result<Vec<T>, Error>>()?;

        return Ok(Box::new(Sequence {
            values,
            position: 0,
            trigger: self.trigger,
        }));
    }
}

impl<T> UnboundValueDecl<T> for SequenceDecl<T>
    where T: Copy + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error> {
        return Ok(Box::new(Sequence {
            values: self.values.clone(),
            position: 0,
            trigger: self.trigger,
        }));
    }
}
