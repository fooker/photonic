use crate::core::Dynamic;
use crate::math;
use std::time::Duration;

pub struct FaderValue {
    limits: (f64, f64),

    value: f64,

    update: Option<f64>,
}

impl FaderValue {
    pub fn new(default_value: f64,
               limits: (f64, f64)) -> Self {
        Self {
            limits,
            value: default_value,
            update: None,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set(&mut self, value: f64) {
        let value = math::clamp(value, self.limits);
        self.update = Some(value);
    }
}

impl Dynamic for FaderValue {
    fn update(&mut self, duration: &Duration) {
        // FIXME: Animation

        if let Some(update) = self.update.take() {
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

    update: Option<()>,
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
            update: None,
        }
    }

    pub fn trigger(&mut self) {
        self.update = Some(());
    }

    pub fn value(&self) -> f64 {
        return match &self.state {
            PushbuttonState::Released => self.released_value,
            PushbuttonState::Pressed(_) => self.pressed_value,
        };
    }
}

impl Dynamic for ButtonValue {
    fn update(&mut self, duration: &Duration) {
        // FIXME: Animation

        if let Some(_) = self.update.take() {
            self.state = PushbuttonState::Pressed(self.hold_time);
        }

        self.state.update(duration);
    }
}

pub enum DynamicValue {
    Fader(FaderValue),
    Button(ButtonValue),
//    Timer(dynamic::Timer),
}

impl DynamicValue {
    pub fn value(&self) -> f64 {
        match self {
            DynamicValue::Fader(ref value) => value.value(),
            DynamicValue::Button(ref value) => value.value(),
        }
    }
}

impl Dynamic for DynamicValue {
    fn update(&mut self, duration: &Duration) {
        match self {
            DynamicValue::Fader(ref mut value) => value.update(duration),
            DynamicValue::Button(ref mut value) => value.update(duration),
        }
    }
}
