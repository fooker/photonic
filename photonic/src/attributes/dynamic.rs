use std::cell::Cell;
use std::time::Duration;
use super::DynamicAttribute;

pub struct Fader {
    name: String,

    limits: (f64, f64),

    value: Cell<f64>,
    update: Cell<Option<f64>>,
}

impl Fader {
    pub fn new(name: &str,
               default_value: f64,
               limits: (f64, f64)) -> Self {
        Self {
            name: name.to_owned(),
            limits,
            value: Cell::new(default_value),
            update: Cell::new(None),
        }
    }

    pub fn set(&self, value: f64) {
        self.update.set(Some(value));
    }
}

impl DynamicAttribute for Fader {
    fn name(&self) -> &str {
        &self.name
    }

    fn value(&self) -> f64 {
        self.value.get()
    }

    fn update(&self, duration: &Duration) {
        // FIXME: Animation

        if let Some(update) = self.update.get().take() {
            self.value.set(update);
        }
    }
}

#[derive(Clone,Copy)]
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

pub struct Pushbutton {
    name: String,

    released_value: f64,
    pressed_value: f64,

    hold_time: Duration,

    state: Cell<PushbuttonState>,

    update: Cell<Option<()>>,
}

impl Pushbutton {
    pub fn new(name: &str,
               released_value: f64,
               pressed_value: f64,
               hold_time: Duration) -> Self {
        Self {
            name: name.to_owned(),
            released_value,
            pressed_value,
            hold_time,
            state: Cell::new(PushbuttonState::Released),
            update: Cell::new(None),
        }
    }

    pub fn trigger(&self) {
        self.update.set(Some(()));
    }
}

impl DynamicAttribute for Pushbutton {
    fn name(&self) -> &str {
        &self.name
    }

    fn value(&self) -> f64 {
        return match &self.state.get() {
            PushbuttonState::Released => self.released_value,
            PushbuttonState::Pressed(_) => self.pressed_value,
        };
    }

    fn update(&self, duration: &Duration) {
        // FIXME: Animation

        if let Some(_) = self.update.get().take() {
            self.state.set(PushbuttonState::Pressed(self.hold_time));
        }

        self.state.update(|state| state.update(duration));
    }
}
