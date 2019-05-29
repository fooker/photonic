use std::fmt;
use std::fmt::Display;
use std::time::Duration;

use failure::ensure;
use failure::Error;
use num::{One, Zero};

use crate::math::Lerp;

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
    where T: Bounded {
    pub fn ensure(&self, value: T) -> Result<T, Error> {
        return value.checked(&self.min, &self.max);
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

pub trait Bounded where Self: Sized {
    fn checked(self, min: &Self, max: &Self) -> Result<Self, Error>;
}

impl<T> Bounded for T where T: PartialOrd + Display {
    fn checked(self, min: &Self, max: &Self) -> Result<Self, Error> {
        ensure!(self >= *min, "Value '{}' below {}", self, min);
        ensure!(self <= *max, "Value '{}' above {}", self, max);

        return Ok(self);
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

pub struct FixedValueDecl<T>(T);

impl<T> UnboundValueDecl<T> for FixedValueDecl<T>
    where T: Copy + 'static {
    fn new(self: Box<Self>) -> Result<Box<Value<T>>, Error> {
        return Ok(Box::new(FixedValue(self.0)));
    }
}

impl<T> BoundValueDecl<T> for FixedValueDecl<T>
    where T: Copy + Bounded + 'static {
    fn new(self: Box<Self>, bounds: Bounds<T>) -> Result<Box<Value<T>>, Error> {
        let value = bounds.ensure(self.0)?;

        return Ok(Box::new(FixedValue(value)));
    }
}

pub trait AsFixedValue<T> {
    fn fixed(self) -> Box<FixedValueDecl<T>>;
}

impl<T, V> AsFixedValue<T> for V
    where V: Copy + 'static,
          T: From<Self> {
    fn fixed(self) -> Box<FixedValueDecl<T>> {
        return Box::new(FixedValueDecl(self.into()));
    }
}

#[derive(Debug, Clone)]
pub struct Range<T>(pub T, pub T);

impl<T> Range<T> {
    pub fn new(v1: T, v2: T) -> Self {
        return Self(v1, v2);
    }
}

impl<T> From<(T, T)> for Range<T> {
    fn from(v: (T, T)) -> Self {
        return Self(v.0, v.1);
    }
}

impl<T> Copy for Range<T>
    where T: Copy {}

impl<T> Bounded for Range<T>
    where T: Bounded {
    fn checked(self, min: &Self, max: &Self) -> Result<Self, Error> {
        return Ok(Self(
            self.0.checked(&min.0, &max.0)?,
            self.1.checked(&min.1, &max.1)?,
        ));
    }
}

impl<T> Lerp for Range<T>
    where T: Lerp {
    fn lerp(a: Self, b: Self, i: f64) -> Self {
        return Self(
            Lerp::lerp(a.0, b.0, i),
            Lerp::lerp(a.1, b.1, i),
        );
    }
}
