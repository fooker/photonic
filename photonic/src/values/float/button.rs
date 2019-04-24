use crate::values::{DynamicValue, Update};
use std::time::Duration;
use crate::math;
use std::sync::mpsc;
use crate::animation::Easing;
use crate::trigger::Timer;

#[derive(Clone, Copy)]
enum ButtonState {
    Released,
    Pressed(Duration),
}

impl ButtonState {
    fn update(&mut self, duration: &Duration) {
        if let ButtonState::Pressed(ref mut remaining) = self {
            if *remaining > *duration {
                *remaining -= *duration;
            } else {
                *self = ButtonState::Released;
            }
        }
    }

    pub fn pressed(&self) -> bool {
        return match self {
            ButtonState::Released => false,
            ButtonState::Pressed(_) => true,
        };
    }
}

pub struct Button {
    value_released: f64,
    value_pressed: f64,

    hold_time: Duration,

    state: ButtonState,
    current: f64,

    auto_trigger: Timer, // TODO: Unify auto_trigger and event input?
    update: (mpsc::SyncSender<()>, mpsc::Receiver<()>),
}

impl Button {
    pub fn new(value_released: f64,
               value_pressed: f64,
               hold_time: Duration,
               auto_trigger: Option<Duration>) -> Self {
        Self {
            value_released,
            value_pressed,
            hold_time,
            state: ButtonState::Released,
            current: value_released,
            auto_trigger: Timer::new(auto_trigger),
            update: mpsc::sync_channel(0),
        }
    }

    pub fn updater(&self) -> mpsc::SyncSender<()> {
        // FIXME: Return trigger lambda function
        return self.update.0.clone();
    }
}

impl DynamicValue<f64> for Button {
    fn get(&self) -> f64 {
        self.current
    }

    fn update(&mut self, duration: &Duration) -> Update<f64> {
        let state_old = self.state.pressed();

        self.state.update(duration);

        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            self.state = ButtonState::Pressed(self.hold_time);
        }

        let state_new = self.state.pressed();

        if (state_old, state_new) == (false, true) {
            self.current = self.value_pressed;
            return Update::Changed(self.current);
        }

        if (state_old, state_new) == (true, false) {
            self.current = self.value_released;
            return Update::Changed(self.current);
        }

        return Update::Idle;
    }
}
