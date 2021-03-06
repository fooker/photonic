use std::fmt;
use std::fmt::Display;
use std::time::Duration;

use anyhow::{ensure, Result};
use num::{One, Zero};

use crate::color::{HSLColor, HSVColor, RGBColor};
use crate::math::Lerp;
use crate::scene::AttrBuilder;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AttrValueType {
    Boolean,
    Integer,
    Decimal,
    Color,
    Range(&'static AttrValueType),
}

pub trait AttrValue: Send + Copy + 'static {
    const TYPE: AttrValueType;
}

pub enum Update<V>
where
    V: AttrValue,
{
    Idle(V),
    Changed(V),
}

impl<V> Update<V>
where
    V: AttrValue,
{
    pub fn value(self) -> V {
        return match self {
            Self::Idle(value) => value,
            Self::Changed(value) => value,
        };
    }
}

pub trait Attr<V>
where
    V: AttrValue,
{
    const KIND: &'static str;

    fn get(&self) -> V;
    fn update(&mut self, duration: Duration) -> Update<V>;
}

/// Inclusive on both sides
pub struct Bounds<V> {
    pub min: V,
    pub max: V,
}

impl<V> Bounds<V>
where
    V: Bounded + Zero + One,
{
    pub fn normal() -> Self {
        return Self {
            min: V::zero(),
            max: V::one(),
        };
    }
}

impl<V> Bounds<V>
where
    V: Bounded,
{
    pub fn ensure(&self, value: V) -> Result<V> {
        return value.checked(&self.min, &self.max);
    }
}

impl<V> Clone for Bounds<V>
where
    V: Clone + Bounded,
{
    fn clone(&self) -> Self {
        return Self {
            min: self.min.clone(),
            max: self.max.clone(),
        };
    }
}

impl<V> Copy for Bounds<V> where V: Copy + Bounded {}

impl<V> fmt::Display for Bounds<V>
where
    V: Bounded + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{}, {}]", self.min, self.max);
    }
}

impl<V> From<(V, V)> for Bounds<V>
where
    V: Bounded,
{
    fn from(bounds: (V, V)) -> Self {
        return Self {
            min: bounds.0,
            max: bounds.1,
        };
    }
}

impl<V> From<Bounds<V>> for (V, V)
where
    V: Bounded,
{
    fn from(bounds: Bounds<V>) -> Self {
        return (bounds.min, bounds.max);
    }
}

pub trait Bounded
where
    Self: Sized,
{
    fn checked(self, min: &Self, max: &Self) -> Result<Self>;
}

impl<V> Bounded for V
where
    V: PartialOrd + Display,
{
    fn checked(self, min: &Self, max: &Self) -> Result<Self> {
        ensure!(self >= *min, "Attribute '{}' below {}", self, min);
        ensure!(self <= *max, "Attribute '{}' above {}", self, max);

        return Ok(self);
    }
}

pub trait UnboundAttrDecl<V>
where
    V: AttrValue,
{
    type Target: Attr<V> + 'static;
    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target>;
}

pub trait BoundAttrDecl<V>
where
    V: AttrValue + Bounded,
{
    type Target: Attr<V> + 'static;
    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target>;
}

pub struct FixedAttr<V>(V)
where
    V: AttrValue;

impl<V> Attr<V> for FixedAttr<V>
where
    V: AttrValue,
{
    const KIND: &'static str = "fixed";

    fn get(&self) -> V {
        return self.0;
    }

    fn update(&mut self, _duration: Duration) -> Update<V> {
        return Update::Idle(self.0);
    }
}

pub struct FixedAttrDecl<V>(V)
where
    V: AttrValue;

impl<V> UnboundAttrDecl<V> for FixedAttrDecl<V>
where
    V: AttrValue,
{
    type Target = FixedAttr<V>;

    fn materialize(self, _builder: &mut AttrBuilder) -> Result<Self::Target> {
        return Ok(FixedAttr(self.0));
    }
}

impl<V> BoundAttrDecl<V> for FixedAttrDecl<V>
where
    V: AttrValue + Bounded,
{
    type Target = FixedAttr<V>;

    fn materialize(self, bounds: Bounds<V>, _builder: &mut AttrBuilder) -> Result<Self::Target> {
        let value = bounds.ensure(self.0)?;

        return Ok(FixedAttr(value));
    }
}

pub trait AsFixedAttr<V>
where
    V: AttrValue,
{
    fn fixed(self) -> FixedAttrDecl<V>;
}

impl<V, T> AsFixedAttr<V> for T
where
    V: AttrValue + From<Self>,
{
    fn fixed(self) -> FixedAttrDecl<V> {
        return FixedAttrDecl(self.into());
    }
}

#[derive(Debug, Clone)]
pub struct Range<V>(pub V, pub V)
where
    V: AttrValue;

impl<V> Range<V>
where
    V: AttrValue,
{
    pub fn new(v1: V, v2: V) -> Self {
        return Self(v1, v2);
    }
}

impl<V> From<(V, V)> for Range<V>
where
    V: AttrValue,
{
    fn from(v: (V, V)) -> Self {
        return Self(v.0, v.1);
    }
}

impl<V> Copy for Range<V> where V: AttrValue {}

impl<V> Bounded for Range<V>
where
    V: AttrValue + Bounded,
{
    fn checked(self, min: &Self, max: &Self) -> Result<Self> {
        return Ok(Self(
            self.0.checked(&min.0, &max.0)?,
            self.1.checked(&min.1, &max.1)?,
        ));
    }
}

impl<V> Range<V>
where
    V: AttrValue + Lerp,
{
    pub fn at(&self, i: f64) -> V {
        return V::lerp(self.0, self.0, i);
    }
}

impl<V> Lerp for Range<V>
where
    V: AttrValue + Lerp,
{
    fn lerp(a: Self, b: Self, i: f64) -> Self {
        return Self(Lerp::lerp(a.0, b.0, i), Lerp::lerp(a.1, b.1, i));
    }
}

impl<V, T> BoundAttrDecl<V> for Box<T>
where
    V: AttrValue + Bounded,
    T: BoundAttrDecl<V>,
{
    type Target = T::Target;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target> {
        return T::materialize(*self, bounds, builder);
    }
}

impl<V, T> UnboundAttrDecl<V> for Box<T>
where
    V: AttrValue,
    T: UnboundAttrDecl<V>,
{
    type Target = T::Target;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target> {
        return T::materialize(*self, builder);
    }
}

impl AttrValue for bool {
    const TYPE: AttrValueType = AttrValueType::Boolean;
}

impl AttrValue for i64 {
    const TYPE: AttrValueType = AttrValueType::Integer;
}

impl AttrValue for f64 {
    const TYPE: AttrValueType = AttrValueType::Decimal;
}

impl<V> AttrValue for Range<V>
where
    V: AttrValue,
{
    const TYPE: AttrValueType = AttrValueType::Range(&V::TYPE);
}

pub trait Color: Copy + Send + 'static {}

impl<C> AttrValue for C
where
    C: Color,
{
    const TYPE: AttrValueType = AttrValueType::Color;
}

impl Color for RGBColor {}

impl Color for HSVColor {}

impl Color for HSLColor {}
