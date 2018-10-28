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

pub mod easing {
    pub fn linear(t: f64) -> f64 { t }

    pub fn in_quad(t: f64) -> f64 { t * t }

    pub fn out_quad(t: f64) -> f64 { t * (2.0 - t) }

    pub fn quad(t: f64) -> f64 { if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t } }

    pub fn in_cubic(t: f64) -> f64 { t * t * t }

    pub fn out_cubic(t: f64) -> f64 { (t - 1.0) * (t - 1.0) * (t - 1.0) + 1.0 }

    pub fn cubic(t: f64) -> f64 { if t < 0.5 { 4.0 * t * t * t } else { (t - 1.0) * (2.0 * t - 2.0) * (2.0 * t - 2.0) + 1.0 } }

    pub fn in_quart(t: f64) -> f64 { t * t * t * t }

    pub fn out_quart(t: f64) -> f64 { 1.0 - (t - 1.0) * (t - 1.0) * (t - 1.0) * (t - 1.0) }

    pub fn quart(t: f64) -> f64 { if t < 0.5 { 8.0 * t * t * t * t } else { 1.0 - 8.0 * (t - 1.0) * (t - 1.0) * (t - 1.0) * (t - 1.0) } }

    pub fn in_quint(t: f64) -> f64 { t * t * t * t * t }

    pub fn out_quint(t: f64) -> f64 { 1.0 + (t - 1.0) * (t - 1.0) * (t - 1.0) * (t - 1.0) * (t - 1.0) }

    pub fn quint(t: f64) -> f64 { if t < 0.5 { 16.0 * t * t * t * t * t } else { 1.0 + 16.0 * (t - 1.0) * (t - 1.0) * (t - 1.0) * (t - 1.0) * (t - 1.0) } }
}
