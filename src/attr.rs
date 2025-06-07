use crate::scene;
pub use bounds::{Bounded, Bounds};
pub use fixed::{AsFixedAttr, FixedAttr, FixedAttrDecl};
pub use range::Range;
pub use values::AttrValue;

pub use self::ext::FreeAttrDeclExt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AttrValueType {
    Never,
    Boolean,
    Integer,
    Decimal,
    Color,
    Range(&'static AttrValueType),
}

impl std::fmt::Display for AttrValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Never => f.write_str("never"),
            Self::Boolean => f.write_str("boolean"),
            Self::Integer => f.write_str("integer"),
            Self::Decimal => f.write_str("decimal"),
            Self::Color => f.write_str("color"),
            Self::Range(element) => write!(f, "range<{element}>"),
        };
    }
}

pub trait Attr<V: AttrValue> {
    fn update(&mut self, ctx: &scene::RenderContext) -> V;
}

#[allow(unreachable_code)]
impl<V> Attr<V> for !
where V: AttrValue
{
    fn update(&mut self, _ctx: &scene::RenderContext) -> V {
        return *self;
    }
}

pub mod bounds;
pub mod ext;
pub mod fixed;
pub mod range;
pub mod values;
