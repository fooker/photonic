use std::time::Duration;

use failure::Error;

use photonic_core::animation::{Animation, Easing, Transition};
use photonic_core::math::Lerp;
use photonic_core::value::*;

pub struct Fader<Input, F>
    where F: Lerp {
    input: Input,
    current: F,

    easing: Easing<f64>,
    transition: Animation<F>,
}

impl<Input, F> Value<F> for Fader<Input, F>
    where F: Lerp + Copy,
          Input: Value<F> {
    fn get(&self) -> F {
        self.current
    }

    fn update(&mut self, duration: &Duration) -> Update<F> {
        if let Update::Changed(next) = self.input.update(duration) {
            self.transition.start(self.easing, self.current, next);
        }

        if let Transition::Running(value) = self.transition.update(duration) {
            self.current = value;
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct FaderDecl<Input> {
    pub input: Input,
    pub easing: Easing<f64>,
}

impl<Input, F> BoundValueDecl<F> for FaderDecl<Input>
    where F: Lerp + Bounded + Copy + 'static,
          Input: BoundValueDecl<F> {
    type Value = Fader<Input::Value, F>;
    fn new(self, bounds: Bounds<F>) -> Result<Self::Value, Error> {
        let input = self.input.new(bounds)?;

        let current = input.get();

        return Ok(Fader {
            input,
            current,
            easing: self.easing,
            transition: Animation::idle(),
        });
    }
}

impl<Input, F> UnboundValueDecl<F> for FaderDecl<Input>
    where F: Lerp + Copy + 'static,
          Input: UnboundValueDecl<F> {
    type Value = Fader<Input::Value, F>;
    fn new(self) -> Result<Self::Value, Error> {
        let input = self.input.new()?;

        let current = input.get();

        return Ok(Fader {
            input,
            current,
            easing: self.easing,
            transition: Animation::idle(),
        });
    }
}
