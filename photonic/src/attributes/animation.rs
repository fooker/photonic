use crate::math::Lerp;
use std::f64::consts::PI;
use std::time::Duration;

#[derive(Clone, Copy)]
pub struct Easing {
    pub func: fn(f64) -> f64,
    pub speed: Duration,
}

pub enum Animation {
    Idle,
    Running(Animator),
}

impl Animation {
    pub fn start(easing: Easing,
                 source: f64,
                 target: f64) -> Animation {
        return Animation::Running(Animator {
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

pub struct Animator {
    easing: Easing,
    source: f64,
    target: f64,
    position: f64,
}

impl Animator {
    fn update(&mut self, duration: &Duration) -> Option<f64> {
        if self.position > 1.0 {
            return None;
        }

        self.position += duration.as_float_secs() / self.easing.speed.as_float_secs();
        return Some(f64::lerp(
            self.source,
            self.target,
            (self.easing.func)(f64::min(1.0, self.position))));
    }
}
