use std::time::Duration;

use failure::Error;
use rand::distributions::uniform::SampleUniform;
use rand::prelude::{FromEntropy, Rng, SmallRng};

use crate::input::{Input, Poll};

use super::*;

pub struct Random<T> {
    bounds: Bounds<T>,

    current: T,

    random: SmallRng,

    trigger: Input<()>,
}

impl<T> Value<T> for Random<T>
    where T: Copy + SampleUniform {
    fn get(&self) -> T {
        self.current
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        if let Poll::Ready(()) = self.trigger.poll() {
            self.current = self.random.gen_range(self.bounds.min, self.bounds.max);
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct RandomDecl {
    pub trigger: Input<()>,
}

impl<T> BoundValueDecl<T> for RandomDecl
    where T: Copy + SampleUniform + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let current = bounds.min; // TODO: Start with a random value?

        return Ok(Box::new(Random {
            bounds,
            current,
            random: SmallRng::from_entropy(),
            trigger: self.trigger,
        }));
    }
}
