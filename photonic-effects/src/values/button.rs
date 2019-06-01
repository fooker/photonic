use std::fmt::Display;
use std::time::Duration;

use failure::Error;

use photonic_core::input::{Input, Poll};
use photonic_core::value::*;

#[derive(Clone, Copy, Debug)]
enum State {
    Released,
    Pressed(Duration),
}

impl State {
    fn update(self, duration: &Duration) -> Self {
        if let State::Pressed(remaining) = self {
            if remaining > *duration {
                return State::Pressed(remaining - *duration);
            } else {
                return State::Released;
            }
        } else {
            return State::Released;
        }
    }

    pub fn pressed(&self) -> bool {
        return match self {
            State::Released => false,
            State::Pressed(_) => true,
        };
    }
}

pub struct Button<T> {
    value_released: T,
    value_pressed: T,

    hold_time: Duration,

    state: State,

    trigger: Input<()>,
}

impl<T> Value<T> for Button<T>
    where T: Copy {
    fn get(&self) -> T {
        return match self.state {
            State::Released => self.value_released,
            State::Pressed(_) => self.value_pressed,
        };
    }

    fn update(&mut self, duration: &Duration) -> Update<T> {
        let state_old = self.state.pressed();

        if let Poll::Ready(()) = self.trigger.poll() {
            self.state = State::Pressed(self.hold_time)
        };

        self.state = self.state.update(duration);

        let state_new = self.state.pressed();

        return match (state_old, state_new) {
            (false, true) => Update::Changed(self.value_pressed),
            (true, false) => Update::Changed(self.value_released),
            _ => Update::Idle,
        };
    }
}

pub struct ButtonDecl<T> {
    pub value: (T, T),
    pub hold_time: Duration,
    pub trigger: Input<()>,
}

impl<T> BoundValueDecl<T> for ButtonDecl<T>
    where T: Copy + Bounded + Display + 'static {
    type Value = Button<T>;
    fn new(self, bounds: Bounds<T>) -> Result<Self::Value, Error> {
        return Ok(Button {
            value_released: bounds.ensure(self.value.0)?,
            value_pressed: bounds.ensure(self.value.1)?,
            hold_time: self.hold_time,
            state: State::Released,
            trigger: self.trigger,
        });
    }
}

impl<T> UnboundValueDecl<T> for ButtonDecl<T>
    where T: Copy + 'static {
    type Value = Button<T>;
    fn new(self) -> Result<Self::Value, Error> {
        return Ok(Button {
            value_released: self.value.0,
            value_pressed: self.value.1,
            hold_time: self.hold_time,
            state: State::Released,
            trigger: self.trigger,
        });
    }
}
