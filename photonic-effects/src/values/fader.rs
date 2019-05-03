use std::time::Duration;

use failure::Error;
use num::Float;

use photonic_core::animation::{Animation, Easing, Transition};
use photonic_core::math::Lerp;
use photonic_core::value::*;

pub struct Fader<F: Float + Lerp> {
    input: Box<Value<F>>,
    current: F,

    easing: Easing<f64>,
    transition: Animation<F>,
}

impl<F: Float + Lerp> Value<F> for Fader<F> {
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

pub struct FaderDecl<F: Float + Lerp> {
    pub input: Box<BoundValueDecl<F>>,
    pub easing: Easing<f64>,
}

impl<F: Float + Lerp> BoundValueDecl<F> for FaderDecl<F>
    where F: 'static {
    fn new(self: Box<Self>, bounds: Bounds<F>) -> Result<Box<Value<F>>, Error> {
        let input = self.input.new(bounds)?;

        let current = input.get();

        return Ok(Box::new(Fader {
            input,
            current,
            easing: self.easing,
            transition: Animation::idle(),
        }));
    }
}
