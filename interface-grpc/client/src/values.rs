use palette::Srgb;
use photonic_interface_grpc_proto::input_value::{ColorRange, DecimalRange, IntegerRange, Rgb};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ValueType {
    Trigger,
    Bool,
    Integer,
    Decimal,
    Color,
    IntegerRange,
    DecimalRange,
    ColorRange,
}

#[derive(Copy, Clone)]
pub struct ColorValue {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FromStr for ColorValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgb = s.parse::<Srgb<u8>>()?;
        let rgb = rgb.into_format::<f32>();

        return Ok(Self {
            r: rgb.red,
            g: rgb.green,
            b: rgb.blue,
        });
    }
}

impl From<ColorValue> for Rgb {
    fn from(val: ColorValue) -> Self {
        return Rgb {
            r: val.r,
            g: val.g,
            b: val.b,
        };
    }
}

pub struct RangeValue<V> {
    pub a: V,
    pub b: V,
}

impl<V> FromStr for RangeValue<V>
where
    V: FromStr + Copy,
    <V as FromStr>::Err: Into<anyhow::Error>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(if let Some((a, b)) = s.split_once("..") {
            Self {
                a: a.parse().map_err(Into::into)?,
                b: b.parse().map_err(Into::into)?,
            }
        } else {
            let s = s.parse().map_err(Into::into)?;
            Self {
                a: s,
                b: s,
            }
        });
    }
}

impl From<RangeValue<i64>> for IntegerRange {
    fn from(val: RangeValue<i64>) -> Self {
        return IntegerRange {
            a: val.a,
            b: val.b,
        };
    }
}

impl From<RangeValue<f32>> for DecimalRange {
    fn from(val: RangeValue<f32>) -> Self {
        return DecimalRange {
            a: val.a,
            b: val.b,
        };
    }
}

impl From<RangeValue<ColorValue>> for ColorRange {
    fn from(val: RangeValue<ColorValue>) -> Self {
        return ColorRange {
            a: Some(val.a.into()),
            b: Some(val.b.into()),
        };
    }
}
