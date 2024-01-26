use std::time::Duration;

pub use bounds::{Bounded, Bounds};
pub use fixed::{AsFixedAttr, FixedAttr, FixedAttrDecl};
pub use range::Range;
pub use values::AttrValue;

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

    // TODO: Take in scene::Context instead of duration
    fn update(&mut self, duration: Duration) -> Self::Value;
}

pub mod values;
pub mod bounds;
pub mod fixed;
pub mod range;
