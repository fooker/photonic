use std::convert::Infallible;

use palette::rgb::Rgb;
use palette::{
    FromColor, Hsl, Hsla, Hsluv, Hsluva, Hsv, Hsva, Hwb, Hwba, Lab, Laba, Lch, Lcha, Lchuv, Lchuva, Luv, Luva, Okhsl,
    Okhsla, Okhsv, Okhsva, Okhwb, Okhwba, Oklab, Oklaba, Oklch, Oklcha, Srgb, Srgba, Xyz, Xyza, Yxy, Yxya,
};

use crate::attr::Range;
use crate::input::trigger::Trigger;

use super::sink::{InputSink, Sink};
use super::{InputValue, InputValueType};

impl super::private::Sealed for Trigger {}

impl InputValue for Trigger {
    const TYPE: InputValueType = InputValueType::Trigger;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Trigger(sink);
    }
}

impl super::private::Sealed for bool {}

impl InputValue for bool {
    const TYPE: InputValueType = InputValueType::Boolean;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Boolean(sink);
    }
}

impl super::private::Sealed for i64 {}

impl InputValue for i64 {
    const TYPE: InputValueType = InputValueType::Integer;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Integer(sink);
    }
}

impl super::private::Sealed for f32 {}

impl InputValue for f32 {
    const TYPE: InputValueType = InputValueType::Decimal;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Decimal(sink);
    }
}

impl super::private::Sealed for Rgb {}

impl InputValue for Rgb {
    const TYPE: InputValueType = InputValueType::Color;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Color(sink);
    }
}

impl super::private::Sealed for Range<i64> {}

impl InputValue for Range<i64> {
    const TYPE: InputValueType = InputValueType::IntegerRange;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::IntegerRange(sink);
    }
}

impl super::private::Sealed for Range<f32> {}

impl InputValue for Range<f32> {
    const TYPE: InputValueType = InputValueType::DecimalRange;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::DecimalRange(sink);
    }
}

impl super::private::Sealed for Range<Rgb> {}

impl InputValue for Range<Rgb> {
    const TYPE: InputValueType = InputValueType::ColorRange;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::ColorRange(sink);
    }
}

pub trait Coerced: Sized {
    type Input: InputValue;
    type Error: std::error::Error + Send + Sync;

    fn try_from_input(input: Self::Input) -> Result<Self, Self::Error>;
}

impl<V> Coerced for Range<V>
where
    V: Coerced,
    Range<V::Input>: InputValue,
{
    type Input = Range<V::Input>;
    type Error = <V as Coerced>::Error;

    fn try_from_input(input: Self::Input) -> Result<Self, Self::Error> {
        return Ok(Range::new(V::try_from_input(input.0)?, V::try_from_input(input.1)?));
    }
}

macro_rules! impl_coerced_from {
    (
        $i:ty =>
        $($t:ty),*
    ) => {
        $(
            impl Coerced for $t {
                type Input = $i;
                type Error = <$t as TryFrom<$i>>::Error;

                fn try_from_input(input: Self::Input) -> Result<Self, Self::Error> {
                    return TryFrom::try_from(input);
                }
            }
        )*
    };
}

macro_rules! impl_coerced_color {
    (
        $i:ty =>
        $($t:ty),*
    ) => {
        $(
            impl Coerced for $t {
                type Input = $i;
                type Error = Infallible;

                fn try_from_input(input: Self::Input) -> Result<Self, Self::Error> {
                    return Ok(FromColor::from_color(input));
                }
            }
        )*
    };
}

impl_coerced_from!(bool => bool);
impl_coerced_from!(i64 => i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
impl_coerced_from!(f32 => f32, f64);
impl_coerced_color!(Rgb => Srgb, Srgba);
impl_coerced_color!(Rgb => Hsv, Hsva);
impl_coerced_color!(Rgb => Hsl, Hsla, Hsluv, Hsluva);
impl_coerced_color!(Rgb => Hwb, Hwba);
impl_coerced_color!(Rgb => Lab, Laba);
impl_coerced_color!(Rgb => Lch, Lcha, Lchuv, Lchuva);
impl_coerced_color!(Rgb => Luv, Luva);
impl_coerced_color!(Rgb => Okhsl, Okhsla);
impl_coerced_color!(Rgb => Okhsv, Okhsva);
impl_coerced_color!(Rgb => Okhwb, Okhwba);
impl_coerced_color!(Rgb => Oklab, Oklaba);
impl_coerced_color!(Rgb => Oklch, Oklcha);
impl_coerced_color!(Rgb => Xyz, Xyza, Yxy, Yxya);
