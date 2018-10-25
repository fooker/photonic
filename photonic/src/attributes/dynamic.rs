use crate::core::Dynamic;
use crate::math;
use std::time::Duration;
use std::sync::mpsc::{self,Receiver,SyncSender};

pub struct FaderValue {
    limits: (f64, f64),

    value: f64,

    update: (SyncSender<f64>, Receiver<f64>),
}

impl FaderValue {
    pub fn new(default_value: f64,
               limits: (f64, f64)) -> Self {
        Self {
            limits,
            value: default_value,
            update: mpsc::sync_channel(0),
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

//    pub fn set(&mut self, value: f64) {
//        let value = math::clamp(value, self.limits);
//        self.update = Some(value);
//    }

    pub fn updater(&self) -> SyncSender<f64> {
        return self.update.0.clone();
    }
}

impl Dynamic for FaderValue {
    fn update(&mut self, duration: &Duration) {
        // FIXME: Animation

        if let Ok(update) = self.update.1.try_recv() {
            self.value = update;
        }
    }
}

#[derive(Clone, Copy)]
enum PushbuttonState {
    Released,
    Pressed(Duration),
}

impl PushbuttonState {
    fn update(self, duration: &Duration) -> Self {
        if let PushbuttonState::Pressed(ref remaining) = self {
            if *remaining > *duration {
                return PushbuttonState::Pressed(*remaining - *duration);
            } else {
                return PushbuttonState::Released;
            }
        } else {
            return self;
        }
    }
}

pub struct ButtonValue {
    released_value: f64,
    pressed_value: f64,

    hold_time: Duration,

    state: PushbuttonState,

    update: (SyncSender<()>, Receiver<()>),
}

impl ButtonValue {
    pub fn new(released_value: f64,
               pressed_value: f64,
               hold_time: Duration) -> Self {
        Self {
            released_value,
            pressed_value,
            hold_time,
            state: PushbuttonState::Released,
            update: mpsc::sync_channel(0),
        }
    }

//    pub fn trigger(&mut self) {
//        self.update = Some(());
//    }

    pub fn value(&self) -> f64 {
        return match &self.state {
            PushbuttonState::Released => self.released_value,
            PushbuttonState::Pressed(_) => self.pressed_value,
        };
    }

    pub fn updater(&self) -> SyncSender<()> {
        return self.update.0.clone();
    }
}

impl Dynamic for ButtonValue {
    fn update(&mut self, duration: &Duration) {
        // FIXME: Animation

        if let Ok(_) = self.update.1.try_recv() {
            self.state = PushbuttonState::Pressed(self.hold_time);
        }

        self.state.update(duration);
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
