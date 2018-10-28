use crate::core::Dynamic;
use crate::math;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::Duration;
use super::animation::*;

pub struct Fader {
    easing: Option<Easing>,

    value: f64,

    animation: Animation,

    update: (SyncSender<f64>, Receiver<f64>),
}

impl Fader {
    pub fn new(initial_value: f64,
               easing: Option<Easing>) -> Self {
        Self {
            easing,
            value: initial_value,
            animation: Animation::Idle,
            update: mpsc::sync_channel(0),
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn updater(&self) -> SyncSender<f64> {
        // FIXME: Return setter lambda function
        return self.update.0.clone();
    }
}

impl Dynamic for Fader {
    fn update(&mut self, duration: &Duration) {
        if let Ok(update) = self.update.1.try_recv() {
            if let Some(easing) = self.easing {
                self.animation = Animation::start(easing, self.value, update);
            } else {
                self.value = update;
            }
        }

        if let Some(value) = self.animation.update(duration) {
            self.value = value;
        }
    }
}

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

    easing_pressed: Option<Easing>,
    easing_released: Option<Easing>,

    state: ButtonState,
    value: f64,

    animation: Animation,

    update: (SyncSender<()>, Receiver<()>),
}

impl Button {
    pub fn new(value_released: f64,
               value_pressed: f64,
               hold_time: Duration,
               easing_pressed: Option<Easing>,
               easing_released: Option<Easing>) -> Self {
        Self {
            value_released,
            value_pressed,
            hold_time,
            easing_pressed,
            easing_released,
            state: ButtonState::Released,
            value: value_released,
            animation: Animation::Idle,
            update: mpsc::sync_channel(0),
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn updater(&self) -> SyncSender<()> {
        // FIXME: Return trigger lambda function
        return self.update.0.clone();
    }
}

impl Dynamic for Button {
    fn update(&mut self, duration: &Duration) {
        let state_old = self.state.pressed();

        self.state.update(duration);
        if let Ok(_) = self.update.1.try_recv() {
            self.state = ButtonState::Pressed(self.hold_time);
        }

        let state_new = self.state.pressed();

        if (state_old, state_new) == (false, true) {
            if let Some(easing) = self.easing_pressed {
                println!("{:?}", easing.speed);
                self.animation = Animation::start(easing, self.value_released, self.value_pressed);
            } else {
                self.value = self.value_pressed;
            }
        }

        if (state_old, state_new) == (true, false) {
            if let Some(easing) = self.easing_released {
                println!("{:?}", easing.speed);
                self.animation = Animation::start(easing, self.value_pressed, self.value_released);
            } else {
                self.value = self.value_released;
            }
        }

        if let Some(value) = self.animation.update(duration) {
            self.value = value;
        }
    }
}

pub struct Sequence {
    values: Vec<f64>,

    duration: Duration,

    easing: Option<Easing>,

    remaining: Duration,
    position: usize,

    value: f64,

    animation: Animation,
}

impl Sequence {
    // TODO: Allow manual switching
    pub fn new(values: Vec<f64>,
               duration: Duration,
               easing: Option<Easing>) -> Self {
        Self {
            values: values.clone(),
            duration,
            easing,
            remaining: duration,
            position: 0,
            value: values[0],
            animation: Animation::Idle,
        }
    }

    pub fn value(&self) -> f64 {
        return self.value;
    }
}

impl Dynamic for Sequence {
    fn update(&mut self, duration: &Duration) {
        if self.remaining < *duration {
            self.remaining += self.duration - *duration;
            self.position = (self.position + 1) % self.values.len();

            if let Some(easing) = self.easing {
                self.animation = Animation::start(easing,
                                                  self.value,
                                                  self.values[self.position]);
            } else {
                self.value = self.values[self.position];
            }
        } else {
            self.remaining -= *duration;
        }

        if let Some(value) = self.animation.update(duration) {
            self.value = value;
        }
    }
}

pub enum DynamicAttribute {
    Fader(Fader),
    Button(Button),
    Sequence(Sequence),
}


impl DynamicAttribute {
    pub fn value(&self) -> f64 {
        match self {
            DynamicAttribute::Fader(ref value) => value.value(),
            DynamicAttribute::Button(ref value) => value.value(),
            DynamicAttribute::Sequence(ref value) => value.value(),
        }
    }
}

impl Dynamic for DynamicAttribute {
    fn update(&mut self, duration: &Duration) {
        match self {
            DynamicAttribute::Fader(ref mut value) => value.update(duration),
            DynamicAttribute::Button(ref mut value) => value.update(duration),
            DynamicAttribute::Sequence(ref mut value) => value.update(duration),
        }
    }
}
