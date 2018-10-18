
//pub mod animation;


use core::*;
use math;
use std::time::Duration;

pub struct DynamicAttribute {
    name: String,
    value: f64,
    target: f64,
}

impl DynamicAttribute {
    pub fn set(&mut self, value: f64) {
        self.target = value;
    }
}

impl Dynamic for DynamicAttribute {
    fn update(&mut self, duration: Duration) {
        // FIXME: Animation

        self.value = self.target;
    }
}

pub enum Attribute {
    Fixed(f64),
    Dynamic(DynamicAttribute),
}

impl Attribute {
    pub fn get(&self) -> f64 {
        match self {
            &Attribute::Fixed(v) => v,
            &Attribute::Dynamic(ref d) => d.value,
        }
    }

    pub fn get_clamped(&self, range: (f64, f64)) -> f64 {
        math::clamp(self.get(), range)
    }
}

impl Dynamic for Attribute {
    fn update(&mut self, duration: Duration) {
        match self {
            &mut Attribute::Fixed(_) => {},
            &mut Attribute::Dynamic(ref mut d) => d.update(duration),
        }
    }
}

impl From<f64> for Attribute {
    fn from(value: f64) -> Self {
        Attribute::Fixed(value)
    }
}
impl From<DynamicAttribute> for Attribute {
    fn from(value: DynamicAttribute) -> Self {
        Attribute::Dynamic(value)
    }
}
