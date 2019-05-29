use std::time::Duration;

use failure::Error;

use photonic_core::input::{Input, Poll};
use photonic_core::value::*;

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

impl<T, V> BoundValueDecl<V> for SequenceDecl<T>
    where V: From<T> + Bounded + Copy + 'static {
    fn new(self: Box<Self>, bounds: Bounds<V>) -> Result<Box<Value<V>>, Error> {
        let values = self.values.into_iter()
            .map(|v| bounds.ensure(v.into()))
            .collect::<Result<Vec<V>, Error>>()?;

        return Ok(Box::new(Sequence {
            values,
            position: 0,
            trigger: self.trigger,
        }));
    }
}

impl<T, V> UnboundValueDecl<V> for SequenceDecl<T>
    where V: From<T> + Copy + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<V>>, Error> {
        let values = self.values.into_iter()
            .map(|v| v.into())
            .collect();

        return Ok(Box::new(Sequence {
            values,
            position: 0,
            trigger: self.trigger,
        }));
    }
}
