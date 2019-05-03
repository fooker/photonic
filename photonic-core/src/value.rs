use failure::ensure;
use failure::Error;
use num::{One, Zero};
use std::fmt;
use std::time::Duration;

pub enum Update<T> {
    Idle,
    Changed(T),
}

pub trait Value<T> {
    fn get(&self) -> T;
    fn update(&mut self, duration: &Duration) -> Update<T>;
}

/// Inclusive on both sides
pub struct Bounds<T> {
    pub min: T,
    pub max: T,
}

pub trait UnboundValueDecl<T> {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error>;
}

impl<T> Bounds<T>
    where T: Zero + One {
    pub fn norm() -> Self {
        return Self {
            min: T::zero(),
            max: T::one(),
        };
    }
}

impl<T> Bounds<T>
    where T: PartialOrd + fmt::Display {
    pub fn ensure(&self, value: T) -> Result<T, Error> {
        ensure!(value >= self.min, "Value '{}' below bound {}", value, self);
        ensure!(value <= self.max, "Value '{}' above bound {}", value, self);

        return Ok(value);
    }
}

impl<T: Clone> Clone for Bounds<T> {
    fn clone(&self) -> Self {
        return Self {
            min: self.min.clone(),
            max: self.max.clone(),
        };
    }
}

impl<T: Copy> Copy for Bounds<T> {}

impl<T> fmt::Display for Bounds<T>
    where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{}, {}]", self.min, self.max);
    }
}

impl<T> From<(T, T)> for Bounds<T> {
    fn from(bounds: (T, T)) -> Self {
        return Self {
            min: bounds.0,
            max: bounds.1,
        };
    }
}

impl<T> Into<(T, T)> for Bounds<T> {
    fn into(self) -> (T, T) {
        return (self.min, self.max);
    }
}

pub trait BoundValueDecl<T> {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error>;
}

struct FixedValue<T>(T);

impl<T> Value<T> for FixedValue<T>
    where T: Copy {
    fn get(&self) -> T {
        return self.0;
    }

    fn update(&mut self, _duration: &Duration) -> Update<T> {
        return Update::Idle;
    }
}

struct FixedValueDecl<T>(T);

impl<T> UnboundValueDecl<T> for FixedValueDecl<T>
    where T: Copy + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error> {
        return Ok(Box::new(FixedValue(self.0)));
    }
}

impl<T> From<T> for Box<UnboundValueDecl<T>>
    where T: Copy + 'static {
    fn from(value: T) -> Self {
        return Box::new(FixedValueDecl(value));
    }
}

impl<T> BoundValueDecl<T> for FixedValueDecl<T>
    where T: Copy + PartialOrd + fmt::Display + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let value = bounds.ensure(self.0)?;

        return Ok(Box::new(FixedValue(value)));
    }
}

impl<T> From<T> for Box<BoundValueDecl<T>>
    where T: Copy + PartialOrd + fmt::Display + 'static {
    fn from(value: T) -> Self {
        return Box::new(FixedValueDecl(value));
    }
}
