use std::time::Duration;

use crate::input::{Input, Poll};

use super::*;

pub struct Manual<T> {
    input: Input<T>,
    current: T,
}

impl<T> Value<T> for Manual<T>
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
    input: Input<T>,
}

impl<T> UnboundValueDecl<T> for ManualDecl<T>
    where T: Copy + Default + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error> {
        return Ok(Box::new(Manual {
            input: self.input,
            current: T::default(),
        }));
    }
}
