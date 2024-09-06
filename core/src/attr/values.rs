use palette::{
    Hsl, Hsla, Hsluv, Hsluva, Hsv, Hsva, Hwb, Hwba, Lab, Laba, Lch, Lcha, Lchuv, Lchuva, Luv, Luva, Okhsl, Okhsla,
    Okhsv, Okhsva, Okhwb, Okhwba, Oklab, Oklaba, Oklch, Oklcha, Srgb, Srgba, Xyz, Xyza, Yxy, Yxya,
};

use super::{AttrValueType, Range};

pub trait AttrValue: Send + Copy + PartialEq + 'static {
    const TYPE: AttrValueType;
}

impl<V> AttrValue for Range<V>
where V: AttrValue
{
    const TYPE: AttrValueType = AttrValueType::Range(&V::TYPE);
}

macro_rules! attr_value {
    (
        $vt:expr
        =>
        $($t:ty),*
    ) => {
        $(
            ::paste::paste! {
                impl AttrValue for $t {
                    const TYPE: AttrValueType = AttrValueType::$vt;
                }
            }
        )*
    };
}

attr_value!(Boolean => bool);
attr_value!(Integer => i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
attr_value!(Decimal => f32, f64);
attr_value!(Color => Srgb, Srgba);
attr_value!(Color => Hsv, Hsva);
attr_value!(Color => Hsl, Hsla, Hsluv, Hsluva);
attr_value!(Color => Hwb, Hwba);
attr_value!(Color => Lab, Laba);
attr_value!(Color => Lch, Lcha, Lchuv, Lchuva);
attr_value!(Color => Luv, Luva);
attr_value!(Color => Okhsl, Okhsla);
attr_value!(Color => Okhsv, Okhsva);
attr_value!(Color => Okhwb, Okhwba);
attr_value!(Color => Oklab, Oklaba);
attr_value!(Color => Oklch, Oklcha);
attr_value!(Color => Xyz, Xyza, Yxy, Yxya);
