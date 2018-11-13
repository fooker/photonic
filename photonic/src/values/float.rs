use crate::animation::*;
use crate::core::*;
use crate::math;
use crate::trigger::*;
use crate::utils;
use rand::prelude::{FromEntropy, Rng, SmallRng};
use std::boxed::FnBox;
use std::cell::Cell;
use std::cell::RefCell;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::Duration;


pub struct Fader {
    min: Option<f64>,
    max: Option<f64>,

    easing: Option<Easing>,

    value: f64,

    animation: Animation,

    update: (SyncSender<f64>, Receiver<f64>),
}

impl Fader {
    pub fn new(min: Option<f64>,
               max: Option<f64>,
               easing: Option<Easing>) -> Self {
        let (min, max) = math::minmax(min, max);

        return Self {
            min,
            max,
            easing,
            value: min.or(max).unwrap_or(0.0),
            animation: Animation::Idle,
            update: mpsc::sync_channel(0),
        };
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn updater(&self) -> SyncSender<f64> {
        // FIXME: Return setter lambda function
        // FIXME: Respect min/max
        return self.update.0.clone();
    }

    pub fn update(&mut self, duration: &Duration) {
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

    auto_trigger: Timer,
    animation: Animation,

    update: (SyncSender<()>, Receiver<()>),
}

impl Button {
    pub fn new(value_released: f64,
               value_pressed: f64,
               hold_time: Duration,
               auto_trigger: Option<Duration>,
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
            auto_trigger: Timer::new(auto_trigger),
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

    pub fn update(&mut self, duration: &Duration) {
        let state_old = self.state.pressed();

        self.state.update(duration);

        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            self.state = ButtonState::Pressed(self.hold_time);
        }

        let state_new = self.state.pressed();

        if (state_old, state_new) == (false, true) {
            if let Some(easing) = self.easing_pressed {
                self.animation = Animation::start(easing, self.value_released, self.value_pressed);
            } else {
                self.value = self.value_pressed;
            }
        }

        if (state_old, state_new) == (true, false) {
            if let Some(easing) = self.easing_released {
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

    easing: Option<Easing>,

    position: usize,
    value: f64,

    auto_trigger: Timer,
    animation: Animation,

    update: (SyncSender<()>, Receiver<()>),
}

impl Sequence {
    // TODO: Allow manual switching
    pub fn new(values: Vec<f64>,
               auto_trigger: Option<Duration>,
               easing: Option<Easing>) -> Self {
        Self {
            values: values.clone(),
            easing,
            position: 0,
            value: values[0],
            auto_trigger: Timer::new(auto_trigger),
            animation: Animation::Idle,
            update: mpsc::sync_channel(0),
        }
    }

    pub fn value(&self) -> f64 {
        return self.value;
    }

    pub fn updater(&self) -> SyncSender<()> {
        // FIXME: Return setter lambda function
        return self.update.0.clone();
    }

    pub fn update(&mut self, duration: &Duration) {
        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            self.position = (self.position + 1) % self.values.len();

            if let Some(easing) = self.easing {
                self.animation = Animation::start(easing,
                                                  self.value,
                                                  self.values[self.position]);
            } else {
                self.value = self.values[self.position];
            }
        }

        if let Some(value) = self.animation.update(duration) {
            self.value = value;
        }
    }
}

pub struct Random {
    min: f64,
    max: f64,

    easing: Option<Easing>,

    value: f64,

    random: SmallRng,

    auto_trigger: Timer,
    animation: Animation,

    update: (SyncSender<()>, Receiver<()>),
}

impl Random {
    pub fn new(min: f64, max: f64,
               auto_trigger: Option<Duration>,
               easing: Option<Easing>) -> Self {
        let (min, max) = math::minmax(min, max);

        return Self {
            min,
            max,
            easing,
            value: min,
            random: SmallRng::from_entropy(),
            auto_trigger: Timer::new(auto_trigger),
            animation: Animation::Idle,
            update: mpsc::sync_channel(0),
        };
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn updater(&self) -> SyncSender<()> {
        // FIXME: Return setter lambda function
        return self.update.0.clone();
    }

    pub fn update(&mut self, duration: &Duration) {
        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            let value = self.random.gen_range(self.min, self.max);

            if let Some(easing) = self.easing {
                self.animation = Animation::start(easing,
                                                  self.value,
                                                  value);
            } else {
                self.value = value;
            }
        }

        if let Some(value) = self.animation.update(duration) {
            self.value = value;
        }
    }
}

pub enum DynamicValue {
    Fader(Fader),
    Button(Button),
    Sequence(Sequence),
    Random(Random),
}


impl DynamicValue {
    pub fn value(&self) -> f64 {
        match self {
            DynamicValue::Fader(ref value) => value.value(),
            DynamicValue::Button(ref value) => value.value(),
            DynamicValue::Sequence(ref value) => value.value(),
            DynamicValue::Random(ref value) => value.value(),
        }
    }

    pub fn update(&mut self, duration: &Duration) {
        match self {
            DynamicValue::Fader(ref mut value) => value.update(duration),
            DynamicValue::Button(ref mut value) => value.update(duration),
            DynamicValue::Sequence(ref mut value) => value.update(duration),
            DynamicValue::Random(ref mut value) => value.update(duration),
        }
    }
}

pub enum Value {
    Fixed(f64),
    Dynamic { name: String, value: DynamicValue },
}

impl Value {
    pub fn new_fixed(value: f64) -> ValueFactory {
        Box::new(move |_| Ok(Value::Fixed(value)))
    }

    fn new_dynamic(name: String, value: DynamicValue) -> Self {
        Value::Dynamic {
            name,
            value,
        }
    }

    pub fn new_fader(name: Option<String>,
                     min: Option<f64>,
                     max: Option<f64>,
                     easing: Option<Easing>) -> ValueFactory {
        Box::new(move |decl: ValueDecl| {
            let name = name.unwrap_or_else(|| decl.name.to_owned());

            let min = utils::combine_opts(decl.min, min, f64::max);
            let max = utils::combine_opts(decl.max, max, f64::min);

            return Ok(Self::new_dynamic(name, DynamicValue::Fader(Fader::new(
                min,
                max,
                easing,
            ))));
        })
    }

    pub fn new_button(name: Option<String>,
                      value_released: Option<f64>,
                      value_pressed: Option<f64>,
                      hold_time: Duration,
                      auto_trigger: Option<Duration>,
                      easing_pressed: Option<Easing>,
                      easing_released: Option<Easing>) -> ValueFactory {
        Box::new(move |decl: ValueDecl| {
            let name = name.unwrap_or_else(|| decl.name.to_owned());

            let value_released = value_released.map(|v| math::clamp_opt(v, (decl.min, decl.max))).or(decl.min);
            let value_pressed = value_pressed.map(|v| math::clamp_opt(v, (decl.min, decl.max))).or(decl.max);

            return Ok(Self::new_dynamic(name, DynamicValue::Button(Button::new(
                value_released.ok_or("value_released is required".to_string())?,
                value_pressed.ok_or("value_pressed is required".to_string())?,
                hold_time,
                auto_trigger,
                easing_pressed,
                easing_released,
            ))));
        })
    }

    pub fn new_sequence(name: Option<String>,
                        values: Vec<f64>,
                        auto_trigger: Option<Duration>,
                        easing: Option<Easing>) -> ValueFactory {
        Box::new(move |decl: ValueDecl| {
            let name = name.unwrap_or_else(|| decl.name.to_owned());

            let values = values.iter()
                               .map(|v| math::clamp_opt(*v, (decl.min, decl.max)))
                               .collect();

            return Ok(Self::new_dynamic(name, DynamicValue::Sequence(Sequence::new(
                values,
                auto_trigger,
                easing,
            ))));
        })
    }

    pub fn new_random(name: Option<String>,
                      min: Option<f64>,
                      max: Option<f64>,
                      auto_trigger: Option<Duration>,
                      easing: Option<Easing>) -> ValueFactory {
        Box::new(move |decl: ValueDecl| {
            let name = name.unwrap_or_else(|| decl.name.to_owned());

            let min = utils::combine_opts(decl.min, min, f64::max);
            let max = utils::combine_opts(decl.max, max, f64::min);

            return Ok(Self::new_dynamic(name, DynamicValue::Random(Random::new(
                min.ok_or("min is required".to_string())?,
                max.ok_or("max is required".to_string())?,
                auto_trigger,
                easing,
            ))));
        })
    }

    pub fn get(&self) -> f64 {
        match self {
            &Value::Fixed(v) => v,
            &Value::Dynamic { ref name, ref value } => value.value(),
        }
    }

    pub fn get_clamped(&self, range: (f64, f64)) -> f64 {
        math::clamp(self.get(), range)
    }

    pub fn update(&mut self, duration: &Duration) {
        match self {
            &mut Value::Fixed(_) => {}
            &mut Value::Dynamic { ref name, ref mut value } => value.update(duration),
        }
    }
}

pub struct ValueDecl {
    pub name: &'static str,

    pub min: Option<f64>,
    pub max: Option<f64>,
}

pub type ValueFactory = Box<FnBox(ValueDecl) -> Result<Value, String>>;
