use std::fmt::Display;
use std::ops;
use anyhow::ensure;
use palette::num::{One, Zero};

// TODO: Replace with ops::Range?

/// Inclusive on both sides
pub struct Bounds<V> {
    pub min: V,
    pub max: V,
}

impl<V> Bounds<V>
    where V: Bounded,
{
    pub fn ensure(&self, value: V) -> anyhow::Result<V> {
        return value.checked(&self.min, &self.max);
    }

    pub fn normal() -> Self
        where V: Zero + One,
    {
        return Self {
            min: V::zero(),
            max: V::one(),
        };
    }
}

impl <V> From<(V, V)> for Bounds<V> {
    fn from((min, max): (V, V)) -> Self {
        return Self {
            min,
            max,
        };
    }
}

impl<V> Clone for Bounds<V>
    where V: Clone,
{
    fn clone(&self) -> Self {
        return Self {
            min: self.min.clone(),
            max: self.max.clone(),
        };
    }
}

impl<V> Copy for Bounds<V> where V: Copy {}

impl<V> Display for Bounds<V>
    where V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "[{}, {}]", self.min, self.max);
    }
}

pub trait Bounded
    where Self: Sized,
{
    fn checked(self, min: &Self, max: &Self) -> anyhow::Result<Self>;
}

impl<V> Bounded for V
    where V: PartialOrd + Display,
{
    fn checked(self, min: &Self, max: &Self) -> anyhow::Result<Self> {
        ensure!(self >= *min, "Attribute '{}' below {}", self, min);
        ensure!(self <= *max, "Attribute '{}' above {}", self, max);

        return Ok(self);
    }
}