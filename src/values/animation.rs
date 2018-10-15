use math::Lerp;
use std::f64::consts::PI;
use std::time::Duration;
use utils::FractionalDuration;

pub type Easing = fn(f64) -> f64;

struct RunningState {
    last: f64,
    next: f64,

    scale: f64,

    position: f64,
}

enum State {
    Idle,
    Running,
}

pub struct Animation {
    func: Easing,
    speed: f64,

//    state: State,
}

impl Animation {
    pub fn new(func: Easing,
               speed: f64,
               value: f64) -> Self {
        Self {
            func,
            speed,
        }
    }

    pub fn start(&mut self, target: f64) {
//        let d = (target - self.value).abs();
//
//        self.state = State::Running(RunningState {
//            last: self.value,
//            next: target,
//            scale: 1.0 / (d / self.speed),
//            position: 0.0,
//        });
    }

    pub fn update(&mut self, duration: Duration) {
//        if let State::Running(ref mut state) = self.state {
//            state.position += duration.as_fractional_secs() * state.scale;
//
//            if state.position >= 1.0 {
//                self.value = state.next;
//                self.state = State::Idle;
//            } else {
//                self.value = f64::lerp(
//                    state.last,
//                    state.next,
//                    state.position);
//            }
//        }
    }
}
