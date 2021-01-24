use std::time::Duration;

use failure::Error;
use num::traits::Num;

use photonic_core::input::{Input, Poll};
use photonic_core::value::*;
use photonic_core::core::SceneBuilder;

pub struct Looper<T>
    where T: Num {
    min: T,
    max: T,
    step: T,

    current: T,

    trigger: Input<()>,
}

impl<T> Value<T> for Looper<T>
    where T: Copy + Num {
    fn get(&self) -> T {
        return self.current;
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        if let Poll::Ready(()) = self.trigger.poll() {
            self.current = self.min + (self.current + self.step - self.min) % (self.max - self.min);
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct LooperDecl<T> {
    pub step: T,
    pub trigger: Input<()>,
}

impl<T> BoundValueDecl<T> for LooperDecl<T>
    where T: Copy + PartialOrd + Num + 'static {
    type Value = Looper<T>;
    fn meterialize(self, bounds: Bounds<T>, mut builder: &mut SceneBuilder) -> Result<Self::Value, Error> {
        let (min, max) = bounds.into();

        let step = if self.step >= T::zero() { self.step } else {
            (self.step % (max - min + T::one())) + (max - min + T::one())
        };

        let max = max + T::one();

        return Ok(Looper {
            min,
            max,
            step,
            current: min,
            trigger: builder.input("trigger", self.trigger)?,
        });
    }
}
