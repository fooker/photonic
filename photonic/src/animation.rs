use crate::math::Lerp;
use std::time::Duration;

pub use ezing::*;

#[derive(Clone, Copy)]
pub struct Easing {
    pub func: fn(f64) -> f64,
    pub speed: Duration,
}

impl Easing {
    pub fn none() -> Option<Self> {
        return None;
    }

    pub fn some(func: fn(f64) -> f64, speed: Duration) -> Option<Self> {
        return Some(Self::with(func, speed));
    }

    pub fn with(func: fn(f64) -> f64, speed: Duration) -> Self {
        return Self {
            func,
            speed,
        };
    }
}

pub enum Animation {
    Idle,
    Running(AnimationRunner),
}

impl Animation {
    pub fn start(easing: Easing,
                 source: f64,
                 target: f64) -> Animation {
        return Animation::Running(AnimationRunner {
            easing,
            source,
            target,
            position: 0.0,
        });
    }

    pub fn update(&mut self, duration: &Duration) -> Option<f64> {
        if let Animation::Running(ref mut animator) = self {
            if let Some(value) = animator.update(duration) {
                return Some(value);
            } else {
                *self = Animation::Idle;
            }
        }

        return None;
    }
}

pub struct AnimationRunner {
    easing: Easing,
    source: f64,
    target: f64,
    position: f64,
}

impl AnimationRunner {
    fn update(&mut self, duration: &Duration) -> Option<f64> {
        if self.position > 1.0 {
            return None;
        }

        self.position += duration.as_secs_f64() / self.easing.speed.as_secs_f64();
        return Some(f64::lerp(
            self.source,
            self.target,
            (self.easing.func)(f64::min(1.0, self.position))));
    }
}
