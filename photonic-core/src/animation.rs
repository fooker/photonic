use std::time::Duration;

pub use ezing::*;
pub use num::Float;

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
        return Self { func, speed };
    }
}

pub struct Animation<F>
where
    F: Lerp,
{
    state: State<F>,
}

impl<F> Animation<F>
where
    F: Lerp + Copy,
{
    pub fn idle() -> Self {
        return Self { state: State::Idle };
    }

    pub fn start(&mut self, easing: Easing<f64>, source: F, target: F) {
        self.state = State::Running(Running {
            easing,
            source,
            target,
            position: 0.0,
        });
    }

    pub fn update(&mut self, duration: Duration) -> Transition<F> {
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

pub enum Transition<F>
where
    F: Lerp,
{
    Idle,
    Running(F),
}

enum State<F>
where
    F: Lerp,
{
    Idle,
    Running(Running<F>),
}

struct Running<F>
where
    F: Lerp,
{
    easing: Easing<f64>,
    source: F,
    target: F,
    position: f64,
}

impl<F> Running<F>
where
    F: Lerp + Copy,
{
    fn update(&mut self, duration: Duration) -> Option<F> {
        if self.position > 1.0 {
            return None;
        }

        self.position += duration.as_secs_f64() / self.easing.speed.as_secs_f64();
        return Some(Lerp::lerp(
            self.source,
            self.target,
            (self.easing.func)(f64::min(1.0, self.position)),
        ));
    }
}
