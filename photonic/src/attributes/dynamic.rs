use crate::core::Dynamic;
use crate::math;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::Duration;
use super::animation::*;

pub struct FaderValue {
    limits: (f64, f64),
    easing: Option<Easing>,

    value: f64,

    animation: Animation,

    update: (SyncSender<f64>, Receiver<f64>),
}

impl FaderValue {
    pub fn new(initial_value: f64,
               limits: (f64, f64),
               easing: Option<Easing>) -> Self {
        Self {
            limits,
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

impl Dynamic for FaderValue {
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

pub struct ButtonValue {
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

impl ButtonValue {
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

impl Dynamic for ButtonValue {
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

pub struct TickerValue {
    delta: f64,
    duration: Duration,
    limits: (f64, f64),

    remaining: Duration,
    value: f64,
}

impl TickerValue {
    pub fn new(delta: f64,
               duration: Duration,
               limits: (f64, f64)) -> Self {
        Self {
            delta,
            duration,
            limits,
            remaining: duration,
            value: limits.0,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Dynamic for TickerValue {
    fn update(&mut self, duration: &Duration) {
        // FIXME: Animation

        if self.remaining < *duration {
            self.value += self.delta;
            self.remaining += self.duration - *duration;
        } else {
            self.remaining -= *duration;
        }
    }
}

pub enum DynamicValue {
    Fader(FaderValue),
    Button(ButtonValue),
    Ticker(TickerValue),
}


impl DynamicValue {
    pub fn value(&self) -> f64 {
        match self {
            DynamicValue::Fader(ref value) => value.value(),
            DynamicValue::Button(ref value) => value.value(),
            DynamicValue::Ticker(ref value) => value.value(),
        }
    }
}

impl Dynamic for DynamicValue {
    fn update(&mut self, duration: &Duration) {
        match self {
            DynamicValue::Fader(ref mut value) => value.update(duration),
            DynamicValue::Button(ref mut value) => value.update(duration),
            DynamicValue::Ticker(ref mut value) => value.update(duration),
        }
    }
}
