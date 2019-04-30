use std::time::Duration;

pub use ezing::*;
use num::Float;

use crate::math::Lerp;

#[derive(Clone, Copy)]
pub struct Easing<F: Float> {
    pub func: fn(F) -> F,
    pub speed: Duration,
}

impl<F: Float> Easing<F> {
    pub fn none() -> Option<Self> {
        return None;
    }

    pub fn some(func: fn(F) -> F, speed: Duration) -> Option<Self> {
        return Some(Self::with(func, speed));
    }

    pub fn with(func: fn(F) -> F, speed: Duration) -> Self {
        return Self {
            func,
            speed,
        };
    }
}

pub struct Animation<F: Float + Lerp> {
    state: State<F>,
}

impl<F: Float + Lerp> Animation<F> {
    pub fn idle() -> Self {
        return Self {
            state: State::Idle,
        };
    }

    pub fn start(&mut self,
                 easing: Easing<f64>,
                 source: F,
                 target: F) {
        self.state = State::Running(Running {
            easing,
            source,
            target,
            position: 0.0,
        });
    }

    pub fn update(&mut self, duration: &Duration) -> Transition<F> {
        match self.state {
            State::Running(ref mut runner) => {
                if let Some(value) = runner.update(duration) {
                    return Transition::Running(value);
                } else {
                    self.state = State::Idle;
                    return Transition::Idle;
                }
            }
            State::Idle => {
                return Transition::Idle;
            }
        }
    }
}

pub enum Transition<F> {
    Idle,
    Running(F),
}

enum State<F: Float + Lerp> {
    Idle,
    Running(Running<F>),
}

struct Running<F: Float + Lerp> {
    easing: Easing<f64>,
    source: F,
    target: F,
    position: f64,
}

impl<F: Float + Lerp> Running<F> {
    fn update(&mut self, duration: &Duration) -> Option<F> {
        if self.position > 1.0 {
            return None;
        }

        self.position = self.position + duration.as_secs_f64() / self.easing.speed.as_secs_f64();
        return Some(F::lerp(
            self.source,
            self.target,
            (self.easing.func)(f64::min(1.0, self.position))));
    }
}
