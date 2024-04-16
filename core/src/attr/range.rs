use anyhow::Result;
use palette::num::{One, Zero};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

use crate::attr::bounds::Bounded;
use crate::math::Lerp;
use crate::AttrValue;

#[derive(Debug, Clone)]
pub struct Range<V>(pub V, pub V);

impl<V> Range<V> {
    pub fn new(v1: V, v2: V) -> Self {
        return Self(v1, v2);
    }

    pub fn point(v: V) -> Self
    where V: Clone {
        return Self(v.clone(), v);
    }
}

impl<V> Range<V> {
    pub fn map<R>(self, f: impl Fn(V) -> R) -> Range<R> {
        return Range(f(self.0), f(self.1));
    }
}

impl<V> From<(V, V)> for Range<V> {
    fn from(v: (V, V)) -> Self {
        return Self(v.0, v.1);
    }
}

impl<V> Copy for Range<V> where V: AttrValue {}

impl<V> Bounded for Range<V>
where V: AttrValue + Bounded
{
    fn checked(self, min: &Self, max: &Self) -> Result<Self> {
        return Ok(Self(self.0.checked(&min.0, &max.0)?, self.1.checked(&min.1, &max.1)?));
    }
}

impl<V> Range<V>
where V: AttrValue + Lerp
{
    pub fn at(&self, i: f32) -> V {
        return V::lerp(self.0, self.0, i);
    }
}

impl<V> Lerp for Range<V>
where V: Lerp
{
    fn lerp(a: Self, b: Self, i: f32) -> Self {
        return Self(Lerp::lerp(a.0, b.0, i), Lerp::lerp(a.1, b.1, i));
    }
}

impl<V> Zero for Range<V>
where V: Zero
{
    fn zero() -> Self {
        return Range(V::zero(), V::zero());
    }
}

impl<V> One for Range<V>
where V: One
{
    fn one() -> Self {
        return Range(V::one(), V::one());
    }
}

impl<V> PartialEq for Range<V>
where V: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        return self.0.eq(&other.0) && self.1.eq(&other.1);
    }
}

impl<V> fmt::Display for Range<V>
where V: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "[{}..{}]", self.0, self.1);
    }
}

impl<V> FromStr for Range<V>
where
    V: FromStr + Clone,
    <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        return Ok(if let Some((a, b)) = s.split_once("..") {
            Range::new(a.parse()?, b.parse()?)
        } else {
            Range::point(s.parse()?)
        });
    }
}

impl<'de, V> Deserialize<'de> for Range<V>
where V: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let (a, b) = <(V, V)>::deserialize(deserializer)?;
        return Ok(Range::new(a, b));
    }
}
