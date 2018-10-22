
pub mod dynamic;
//pub mod animation;


use crate::core::*;
use crate::math;
use std::time::Duration;
use std::cell::Cell;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

pub trait DynamicAttribute {
    fn name(&self) -> &str;
    fn value(&self) -> f64;

    fn update(&self, duration: &Duration);
}

//impl DynamicAttribute {
//    pub fn new(name: &str, behavior: Box<Behavior>) -> Self {
//        Self {
//            name: name.to_owned(),
//            value: 0.0, //FIXME: Get default value from behavior
//            behavior,
//        }
//    }
//
//    pub fn name(&self) -> &str {
//        &self.name
//    }
//
//    pub fn value(&self) -> f64 {
//        self.value
//    }
//
//    pub fn behavior(&self) -> Rc<RefCell<Box<Behavior>>> {
//        self.behavior.clone()
//    }
//}

//impl Dynamic for DynamicAttribute {
//    fn update(&mut self, duration: &Duration) {
//        self.value = self.behavior.update(duration);
//    }
//}

pub enum Attribute {
    Fixed(f64),
    Dynamic(Rc<Box<DynamicAttribute>>),
}

impl Attribute {
    pub fn get(&self) -> f64 {
        match self {
            &Attribute::Fixed(v) => v,
            &Attribute::Dynamic(ref d) => d.value(),
        }
    }

    pub fn get_clamped(&self, range: (f64, f64)) -> f64 {
        math::clamp(self.get(), range)
    }
}

impl Dynamic for Attribute {
    fn update(&mut self, duration: &Duration) {
        match self {
            &mut Attribute::Fixed(_) => {},
            &mut Attribute::Dynamic(ref d) => d.update(duration),
        }
    }
}

impl From<f64> for Attribute {
    fn from(value: f64) -> Self {
        Attribute::Fixed(value)
    }
}
impl From<Box<DynamicAttribute>> for Attribute {
    fn from(value: Box<DynamicAttribute>) -> Self {
        Attribute::Dynamic(Rc::new(value))
    }
}
