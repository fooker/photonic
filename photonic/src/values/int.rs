use crate::core::*;
use crate::math;
use crate::trigger::*;
use crate::utils;
use rand::prelude::{FromEntropy, Rng, SmallRng};
use std::boxed::FnBox;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::Duration;

pub struct Manual {
    min: Option<i64>,
    max: Option<i64>,

//    value: i64,

    update: (SyncSender<i64>, Receiver<i64>),
}

impl Manual {
    pub fn new(min: Option<i64>,
               max: Option<i64>) -> Self {
        let (min, max) = math::minmax(min, max);

        return Self {
            min,
            max,
//            value: min.or(max).unwrap_or(0),
            update: mpsc::sync_channel(0),
        };
    }

//    pub fn value(&self) -> i64 {
//        self.value
//    }

    pub fn updater(&self) -> SyncSender<i64> {
        // FIXME: Return setter lambda function
        // FIXME: Respect min/max
        return self.update.0.clone();
    }

    pub fn update(&mut self, duration: &Duration) -> Option<i64> {
        if let Ok(update) = self.update.1.try_recv() {
//            self.value = update;
            return Some(update);
        } else {
            return None;
        }
    }
}

pub struct Loop {
    min: i64,
    max: i64,
    step: i64,

    current: i64,

    auto_trigger: Timer,
    update: (SyncSender<()>, Receiver<()>),
}

impl Loop {
    pub fn new(min: i64,
               max: i64,
               step: i64,
               auto_trigger: Option<Duration>) -> Self {
        let (min, max) = math::minmax(min, max);

        let step = if step >= 0 { step } else {
            (step % (max - min + 1)) + (max - min + 1)
        };

        let max = max + 1;

        Self {
            min,
            max,
            step,
            current: min,
            auto_trigger: Timer::new(auto_trigger),
            update: mpsc::sync_channel(0),
        }
    }

//    pub fn value(&self) -> i64 {
//        self.value
//    }

    pub fn updater(&self) -> SyncSender<()> {
        // FIXME: Return setter lambda function
        return self.update.0.clone();
    }

    pub fn update(&mut self, duration: &Duration) -> Option<i64> {
        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            self.current = self.min + (self.current + self.step - self.min) % (self.max - self.min);
            return Some(self.current);
        } else {
            return None;
        }
    }
}

pub struct Sequence {
    values: Vec<i64>,

    position: usize,
//    value: i64,

    auto_trigger: Timer,
    update: (SyncSender<()>, Receiver<()>),
}

impl Sequence {
    // TODO: Allow manual switching
    pub fn new(values: Vec<i64>,
               auto_trigger: Option<Duration>) -> Self {
        Self {
            values: values.clone(),
            position: 0,
//            value: values[0],
            auto_trigger: Timer::new(auto_trigger),
            update: mpsc::sync_channel(0),
        }
    }

//    pub fn value(&self) -> i64 {
//        return self.value;
//    }

    pub fn update(&mut self, duration: &Duration) -> Option<i64> {
        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            // TODO: Unify manual and automatic triggers
            self.position = (self.position + 1) % self.values.len();
            return Some(self.values[self.position]);
        } else {
            return None;
        }
    }
}

pub struct Random {
    min: i64,
    max: i64,

//    value: i64,

    random: SmallRng,

    auto_trigger: Timer,
    update: (SyncSender<()>, Receiver<()>),
}

impl Random {
    pub fn new(min: i64,
               max: i64,
               auto_trigger: Option<Duration>) -> Self {
        let (min, max) = math::minmax(min, max);

        return Self {
            min,
            max,
//            value: min,
            random: SmallRng::from_entropy(),
            auto_trigger: Timer::new(auto_trigger),
            update: mpsc::sync_channel(0),
        };
    }

//    pub fn value(&self) -> i64 {
//        self.value
//    }

    pub fn updater(&self) -> SyncSender<()> {
        // FIXME: Return setter lambda function
        return self.update.0.clone();
    }

    pub fn update(&mut self, duration: &Duration) -> Option<i64> {
        if self.auto_trigger.update(duration) || self.update.1.try_recv().is_ok() {
            return Some(self.random.gen_range(self.min, self.max));
        } else {
            return None;
        }
    }
}

pub enum DynamicValue {
    Manual(Manual),
    Loop(Loop),
    Sequence(Sequence),
    Random(Random),
}

impl DynamicValue {
//    pub fn value(&self) -> i64 {
//        match self {
//            DynamicValue::Manual(ref value) => value.value(),
//            DynamicValue::Loop(ref value) => value.value(),
//            DynamicValue::Sequence(ref value) => value.value(),
//            DynamicValue::Random(ref value) => value.value(),
//        }
//    }

    fn update(&mut self, duration: &Duration) -> Option<i64> {
        match self {
            DynamicValue::Manual(ref mut value) => value.update(duration),
            DynamicValue::Loop(ref mut value) => value.update(duration),
            DynamicValue::Sequence(ref mut value) => value.update(duration),
            DynamicValue::Random(ref mut value) => value.update(duration),
        }
    }
}

pub enum Value {
    Fixed(i64),
    Dynamic { name: String, value: DynamicValue },
}

impl Value {
    pub fn new_fixed(value: i64) -> ValueFactory {
        Box::new(move |_| Ok(Value::Fixed(value)))
    }

    fn new_dynamic(name: String, value: DynamicValue) -> Self {
        Value::Dynamic {
            name,
            value,
        }
    }

    pub fn new_manual(name: Option<String>,
                      min: Option<i64>,
                      max: Option<i64>) -> ValueFactory {
        Box::new(move |decl: ValueDecl| {
            let name = name.unwrap_or_else(|| decl.name.to_owned());

            let min = utils::combine_opts(decl.min, min, i64::max);
            let max = utils::combine_opts(decl.max, max, i64::min);

            return Ok(Self::new_dynamic(name, DynamicValue::Manual(Manual::new(
                min,
                max,
            ))));
        })
    }

    pub fn new_loop(name: Option<String>,
                    min: Option<i64>,
                    max: Option<i64>,
                    step: i64,
                    auto_trigger: Option<Duration>) -> ValueFactory {
        Box::new(move |decl: ValueDecl| {
            let name = name.unwrap_or_else(|| decl.name.to_owned());

            let min = utils::combine_opts(decl.min, min, i64::max);
            let max = utils::combine_opts(decl.max, max, i64::min);

            return Ok(Self::new_dynamic(name, DynamicValue::Loop(Loop::new(
                min.ok_or("min is required".to_string())?,
                max.ok_or("max is required".to_string())?,
                step,
                auto_trigger,
            ))));
        })
    }

    pub fn new_sequence(name: Option<String>,
                        values: Vec<i64>,
                        auto_trigger: Option<Duration>) -> ValueFactory {
        Box::new(move |decl: ValueDecl| {
            let name = name.unwrap_or_else(|| decl.name.to_owned());

            let values = values.iter()
                               .map(|v| math::clamp_opt(*v, (decl.min, decl.max)))
                               .collect();

            return Ok(Self::new_dynamic(name, DynamicValue::Sequence(Sequence::new(
                values,
                auto_trigger,
            ))));
        })
    }

    pub fn new_random(name: Option<String>,
                      min: Option<i64>,
                      max: Option<i64>,
                      auto_trigger: Option<Duration>) -> ValueFactory {
        Box::new(move |decl: ValueDecl| {
            let name = name.unwrap_or_else(|| decl.name.to_owned());

            let min = utils::combine_opts(decl.min, min, i64::max);
            let max = utils::combine_opts(decl.max, max, i64::min);

            return Ok(Self::new_dynamic(name, DynamicValue::Random(Random::new(
                min.ok_or("min is required".to_string())?,
                max.ok_or("max is required".to_string())?,
                auto_trigger,
            ))));
        })
    }

//    pub fn get(&self) -> i64 {
//        match self {
//            &Value::Fixed(v) => v,
//            &Value::Dynamic { ref name, ref value } => value.value(),
//        }
//    }
//
//    pub fn get_clamped(&self, range: (i64, i64)) -> i64 {
//        math::clamp(self.get(), range)
//    }

    pub fn update(&mut self, duration: &Duration) -> Option<i64> {
        match self {
            Value::Fixed(_) => {
                return None;
            }

            Value::Dynamic { ref name, ref mut value } => {
                return value.update(duration);
            }
        }
    }
}

pub struct ValueDecl {
    pub name: &'static str,

    pub min: Option<i64>,
    pub max: Option<i64>,
}

pub type ValueFactory = Box<FnBox(ValueDecl) -> Result<Value, String>>;
