use palette::{Hsl, Hsv, Srgb};
use super::{AttrValueType, Range};

pub trait AttrValue: Send + Copy + 'static {
    const TYPE: AttrValueType;
}

impl AttrValue for bool {
    const TYPE: AttrValueType = AttrValueType::Boolean;
}

impl AttrValue for i64 {
    const TYPE: AttrValueType = AttrValueType::Integer;
}

impl AttrValue for f32 {
    const TYPE: AttrValueType = AttrValueType::Decimal;
}

impl AttrValue for f64 {
    const TYPE: AttrValueType = AttrValueType::Decimal;
}

impl<V> AttrValue for Range<V>
    where V: AttrValue,
{
    const TYPE: AttrValueType = AttrValueType::Range(&V::TYPE);
}

impl AttrValue for Srgb {
    const TYPE: AttrValueType = AttrValueType::Color;
}

impl AttrValue for Hsv {
    const TYPE: AttrValueType = AttrValueType::Color;
}

impl AttrValue for Hsl {
    const TYPE: AttrValueType = AttrValueType::Color;
}
