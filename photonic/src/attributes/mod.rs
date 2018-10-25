use crate::core::*;
use crate::math;
pub use self::dynamic::DynamicValue;
use std::cell::Cell;
use std::cell::RefCell;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub mod dynamic;
//pub mod animation;


pub enum Attribute {
    Fixed(f64),
    Dynamic { name: String, value: DynamicValue },
}

impl Attribute {
    pub fn new_fixed(value: f64) -> Self {
        Attribute::Fixed(value)
    }

    pub fn new_dynamic<S: AsRef<str>>(name: S, value: DynamicValue) -> Self {
        Attribute::Dynamic {
            name: name.as_ref().to_owned(),
            value,
        }
    }

    pub fn get(&self) -> f64 {
        match self {
            &Attribute::Fixed(v) => v,
            &Attribute::Dynamic { ref name, ref value } => value.value(),
        }
    }

    pub fn get_clamped(&self, range: (f64, f64)) -> f64 {
        math::clamp(self.get(), range)
    }
}

impl Dynamic for Attribute {
    fn update(&mut self, duration: &Duration) {
        match self {
            &mut Attribute::Fixed(_) => {}
            &mut Attribute::Dynamic { ref name, ref mut value } => value.update(duration),
        }
    }
}
