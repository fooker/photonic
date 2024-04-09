use anyhow::bail;
use palette::Srgb;
use photonic_interface_grpc_proto::input_value::{ColorRange, DecimalRange, IntegerRange, Rgb};
use std::str::FromStr;

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

impl Into<Rgb> for ColorValue {
    fn into(self) -> Rgb {
        return Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        };
    }
}

pub struct RangeValue<V> {
    pub a: V,
    pub b: V,
}

impl<V> FromStr for RangeValue<V>
where
    V: FromStr,
    <V as FromStr>::Err: Into<anyhow::Error>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once("..") {
            return Ok(Self {
                a: a.parse().map_err(Into::into)?,
                b: b.parse().map_err(Into::into)?,
            });
        } else {
            bail!("Not a range");
        }
    }
}

impl Into<IntegerRange> for RangeValue<i64> {
    fn into(self) -> IntegerRange {
        return IntegerRange {
            a: self.a,
            b: self.b,
        };
    }
}

impl Into<DecimalRange> for RangeValue<f32> {
    fn into(self) -> DecimalRange {
        return DecimalRange {
            a: self.a.into(),
            b: self.b.into(),
        };
    }
}

impl Into<ColorRange> for RangeValue<ColorValue> {
    fn into(self) -> ColorRange {
        return ColorRange {
            a: Some(self.a.into()),
            b: Some(self.b.into()),
        };
    }
}
