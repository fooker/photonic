use std::time::Duration;

use crate::animation::{Animation, Easing};
use crate::input::{Input, Poll};

use super::*;

// TODO: Generalize this to Num or Float?

pub struct Fader {
    bounds: Bounds<f64>,

    input: Box<Value<f64>>,
    current: f64,

    easing: Easing,
    transition: Animation,
}

impl Value<f64> for Fader {
    fn get(&self) -> f64 {
        self.current
    }

    fn update(&mut self, duration: &Duration) -> Update<f64> {
        if let Update::Changed(next) = self.input.update(duration) {
            self.transition = Animation::start(self.easing, self.current, next);
        }

        if let Some(value) = self.transition.update(duration) {
            self.current = value;
            return Update::Changed(self.current);
        } else {
            return Update::Idle;
        }
    }
}

pub struct FaderDecl {
    pub input: Box<BoundValueDecl<f64>>,
    pub easing: Easing,
}

impl BoundValueDecl<f64> for FaderDecl {
    fn new(self: Box<Self>, bounds: Bounds<f64>) -> Result<Box<Value<f64>>, Error> {
        let input = self.input.new(bounds)?;

        let current = input.get();

        return Ok(Box::new(Fader {
            bounds,
            input,
            current,
            easing: self.easing,
            transition: Animation::Idle,
        }));
    }
}
