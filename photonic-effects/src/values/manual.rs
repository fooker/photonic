use std::fmt::Display;
use std::time::Duration;

use failure::Error;

use photonic_core::input::{Input, Poll};
use photonic_core::value::*;
use photonic_core::core::SceneBuilder;

pub struct BoundManual<T> {
    bounds: Bounds<T>,

    value: Input<T>,
    current: T,
}

impl<T> Value<T> for BoundManual<T>
    where T: Copy + Bounded + Display {
    fn get(&self) -> T {
        self.current
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        if let Poll::Ready(update) = self.value.poll() {
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
    value: Input<T>,
    current: T,
}

impl<T> Value<T> for UnboundManual<T>
    where T: Copy {
    fn get(&self) -> T {
        self.current
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        if let Poll::Ready(update) = self.value.poll() {
            self.current = update;
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct ManualDecl<T> {
    pub value: Input<T>,
}

impl<T> From<Input<T>> for ManualDecl<T> {
    fn from(value: Input<T>) -> Self {
        return Self { value };
    }
}

impl<T> BoundValueDecl<T> for ManualDecl<T>
    where T: Copy + Bounded + Display + 'static {
    type Value = BoundManual<T>;

    fn meterialize(self, bounds: Bounds<T>, mut builder: &mut SceneBuilder) -> Result<Self::Value, Error> {
        let value = builder.input("value", self.value)?;

        let current = bounds.min;

        return Ok(BoundManual {
            bounds,
            value,
            current,
        });
    }
}

impl<T> UnboundValueDecl<T> for ManualDecl<T>
    where T: Copy + Default + 'static {
    type Value = UnboundManual<T>;

    fn meterialize(self, mut builder: &mut SceneBuilder) -> Result<Self::Value, Error> {
        let value = builder.input("value", self.value)?;

        let current = T::default();

        return Ok(UnboundManual {
            value,
            current,
        });
    }
}
