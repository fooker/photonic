use std::time::Duration;

use anyhow::Result;

pub use bounds::{Bounded, Bounds};
pub use fixed::{AsFixedAttr, FixedAttr, FixedAttrDecl};
pub use range::Range;
pub use values::AttrValue;

use crate::AttrBuilder;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AttrValueType {
    Boolean,
    Integer,
    Decimal,
    Color,
    Range(&'static AttrValueType),
}

impl std::fmt::Display for AttrValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Boolean => f.write_str("boolean"),
            Self::Integer => f.write_str("integer"),
            Self::Decimal => f.write_str("decimal"),
            Self::Color => f.write_str("color"),
            Self::Range(element) => write!(f, "range<{}>", element),
        };
    }
}

pub trait Attr {
    type Value: AttrValue;

    const KIND: &'static str;

    fn update(&mut self, duration: Duration) -> Self::Value;
}

pub trait FreeAttrDecl {
    type Value: AttrValue;
    type Target: Attr<Value=Self::Value>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target>;
}

pub trait BoundAttrDecl {
    type Value: AttrValue + Bounded;
    type Target: Attr<Value=Self::Value>;

    fn materialize(self, bounds: Bounds<Self::Value>, builder: &mut AttrBuilder) -> Result<Self::Target>;
}

impl<V, T> BoundAttrDecl for Box<T>
    where V: AttrValue + Bounded,
          T: BoundAttrDecl<Value=V>,
{
    type Value = V;
    type Target = T::Target;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target> {
        return T::materialize(*self, bounds, builder);
    }
}

impl<V, T> FreeAttrDecl for Box<T>
    where V: AttrValue,
          T: FreeAttrDecl<Value=V>,
{
    type Value = V;
    type Target = T::Target;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target> {
        return T::materialize(*self, builder);
    }
}

pub mod values;
pub mod bounds;
pub mod fixed;
pub mod range;

