use photonic_core::color::RGBColor;
use crate::RenderContext;

pub trait Color: Sized {
    fn transform(color: Self, context: &RenderContext<Self>) -> Self;
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "dyn", derive(serde::Deserialize))]
pub struct RGB {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color for RGB {
    fn transform(color: Self, context: &RenderContext<Self>) -> Self {
        // Apply brightness
        let color = Self {
            red: color.red * context.brightness,
            green: color.green * context.brightness,
            blue: color.blue * context.brightness,
        };

        // Apply gamma linearization
        let color = if let Some(gamma_factor) = context.gamma_factor {
            Self {
                red: f64::powf(color.red, gamma_factor),
                green: f64::powf(color.green, gamma_factor),
                blue: f64::powf(color.blue, gamma_factor),
            }
        } else { color };

        // Apply correction
        let color = if let Some(correction) = context.correction {
            Self {
                red: color.red * correction.red,
                green: color.green * correction.green,
                blue: color.blue * correction.blue,
            }
        } else { color };

        return color;
    }
}

impl From<RGBColor> for RGB {
    fn from(color: RGBColor) -> Self {
        return Self {
            red: color.red,
            green: color.green,
            blue: color.blue,
        };
    }
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "dyn", derive(serde::Deserialize))]
pub struct RGBW {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub white: f64,
}

impl Color for RGBW {
    fn transform(color: Self, context: &RenderContext<Self>) -> Self {
        // Apply brightness
        let color = Self {
            red: color.red * context.brightness,
            green: color.green * context.brightness,
            blue: color.blue * context.brightness,
            white: color.white * context.brightness,
        };

        // Apply gamma linearization
        let color = if let Some(gamma_factor) = context.gamma_factor {
            Self {
                red: f64::powf(color.red, gamma_factor),
                green: f64::powf(color.green, gamma_factor),
                blue: f64::powf(color.blue, gamma_factor),
                white: f64::powf(color.white, gamma_factor),
            }
        } else { color };

        // Apply correction
        let color = if let Some(correction) = context.correction {
            Self {
                red: color.red * correction.red,
                green: color.green * correction.green,
                blue: color.blue * correction.blue,
                white: color.white * correction.white,
            }
        } else { color };

        return color;
    }
}

impl From<RGBColor> for RGBW {
    fn from(color: RGBColor) -> Self {
        return Self {
            red: color.red,
            green: color.green,
            blue: color.blue,
            white: 0.0,
        };
    }
}

pub trait Chip {
    type Element: Color;

    type Channels: IntoIterator<Item=f64>;

    const CHANNELS: usize;

    fn expand(element: Self::Element) -> Self::Channels;
}

pub trait Offset<C, const N: usize> {
    fn get(element: C) -> f64;
}

macro_rules! impl_offset {
    ($name:ident<$element:ty>[$n:literal] => $e:ident) => {
        impl Offset<$element, $n> for $name {
            fn get(element: $element) -> f64 { element.$e }
        }
    };
}

macro_rules! impl_chip {
    ($name:ident<$element:ty> => $element0:ident, $element1:ident, $element2:ident) => {
        pub struct $name;
        
        impl_offset!($name<$element>[0] => $element0);
        impl_offset!($name<$element>[1] => $element1);
        impl_offset!($name<$element>[2] => $element2);
        
        impl Chip for $name {
            type Element = $element;
            type Channels = [f64; 3];

            const CHANNELS: usize = 3;
            
            fn expand(element: Self::Element) -> [f64; 3] {
                return [
                    <Self as Offset::<Self::Element, 0>>::get(element),
                    <Self as Offset::<Self::Element, 1>>::get(element),
                    <Self as Offset::<Self::Element, 2>>::get(element),
                ];
            }
        }
    };

    ($name:ident<$element:ty> => $element0:ident, $element1:ident, $element2:ident, $element3:ident) => {
        pub struct $name;

        impl_offset!($name<$element>[0] => $element0);
        impl_offset!($name<$element>[1] => $element1);
        impl_offset!($name<$element>[2] => $element2);
        impl_offset!($name<$element>[3] => $element3);

        impl Chip for $name {
            type Element = $element;
            type Channels = [f64; 4];

            const CHANNELS: usize = 4;

            fn expand(element: Self::Element) -> [f64; 4] {
                return [
                    <Self as Offset::<Self::Element, 0>>::get(element),
                    <Self as Offset::<Self::Element, 1>>::get(element),
                    <Self as Offset::<Self::Element, 2>>::get(element),
                    <Self as Offset::<Self::Element, 3>>::get(element),
                ];
            }
        }
    };
}

impl_chip!(Ws2811Rgb<RGB> => red, green, blue);
impl_chip!(Ws2811Rbg<RGB> => red, blue, green);
impl_chip!(Ws2811Grb<RGB> => green, blue, red);
impl_chip!(Ws2811Gbr<RGB> => green, red, blue);
impl_chip!(Ws2811Brg<RGB> => blue, red, green);
impl_chip!(Ws2811Bgr<RGB> => blue, green, red);

impl_chip!(Sk6812Rgbw<RGBW> => red, green, blue, white);
impl_chip!(Sk6812Rbgw<RGBW> => red, blue, green, white);
impl_chip!(Sk6812Gbrw<RGBW> => green, blue, red, white);
impl_chip!(Sk6812Grbw<RGBW> => green, red, blue, white);
impl_chip!(Sk6812Brgw<RGBW> => blue, red, green, white);
impl_chip!(Sk6812Bgrw<RGBW> => blue, green, red, white);

pub type Ws2812 = Ws2811Grb;
pub type Sk6812 = Ws2811Grb;

pub type Sk6812W = Sk6812Grbw;
