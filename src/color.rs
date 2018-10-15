use core::MainColor;
use math;

pub use scarlet::color::Color;

pub trait Black {
    fn black() -> Self;
}

impl math::Lerp for MainColor {
    fn lerp(c1: Self, c2: Self, f: f64) -> Self {
        use scarlet::colorpoint::ColorPoint;

        ColorPoint::weighted_midpoint(c2, c1, f)
    }
}

impl Black for MainColor {
    fn black() -> Self {
        MainColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

//trait FromColor: From<RGB> + From<HSV> + From<HSL> {}
//
//#[derive(Debug, Clone, Copy, Default)]
//pub struct RGB {
//    pub r: f64,
//    pub g: f64,
//    pub b: f64,
//}
//
//impl RGB {
//    pub fn new(r: f64, g: f64, b: f64) -> Self {
//        Self { r, g, b }
//    }
//
//    pub fn rgb(&self) -> (f64, f64, f64) {
//        (self.r, self.g, self.b)
//    }
//
//    pub fn rgb8(&self) -> (u8, u8, u8) {
//        (
//            (self.r * 255.0) as u8,
//            (self.g * 255.0) as u8,
//            (self.b * 255.0) as u8,
//        )
//    }
//}
//
//impl FromColor for RGB {}
//
//impl From<HSV> for RGB {
//    fn from(color: HSV) -> Self {
//        let c = color.value * color.saturation;
//        let h = color.hue / 60.0;
//        let x =
//    }
//}
//
//impl From<HSL> for RGB {
//    fn from(color: HSL) -> Self {
//        unimplemented!()
//    }
//}
//
//impl math::Lerp for RGB {
//    fn lerp(c1: Self, c2: Self, i: f64) -> Self {
//        assert!(0.0 <= i && i <= 1.0);
//
//        Self {
//            r: f64::lerp(c1.r, c2.r, i),
//            g: f64::lerp(c1.g, c2.g, i),
//            b: f64::lerp(c1.b, c2.b, i),
//        }
//    }
//}
//
//
//#[derive(Debug, Clone, Copy, Default)]
//pub struct HSV {
//    pub hue: f64,
//    pub saturation: f64,
//    pub value: f64,
//}
//
//impl HSV {
//    pub fn new(hue: f64, saturation: f64, value: f64) -> Self {
//        Self { hue, saturation, value }
//    }
//}
//
//impl FromColor for HSV {}
//
//impl From<RGB> for HSV {
//    fn from(color: RGB) -> Self {
//        unimplemented!()
//    }
//}
//
//impl From<HSL> for HSV {
//    fn from(color: HSL) -> Self {
//        unimplemented!()
//    }
//}
//
//
//#[derive(Debug, Clone, Copy, Default)]
//pub struct HSL {
//    pub hue: f64,
//    pub saturation: f64,
//    pub lightness: f64,
//}
//
//impl HSL {
//    pub fn new(hue: f64, saturation: f64, lightness: f64) -> Self {
//        Self { hue, saturation, lightness }
//    }
//}
//
//impl FromColor for HSL {}
//
//impl From<RGB> for HSL {
//    fn from(color: RGB) -> Self {
//        unimplemented!()
//    }
//}
//
//impl From<HSV> for HSL {
//    fn from(color: HSV) -> Self {
//        unimplemented!()
//    }
//}
//
//
//#[derive(Debug, Clone, Copy, Default)]
//pub struct RGB8 {
//    pub r: u8,
//    pub g: u8,
//    pub b: u8,
//}
//
//impl RGB8 {
//    pub fn new(r: u8, g: u8, b: u8) -> Self {
//        Self { r, g, b }
//    }
//}
//
//impl From<RGB> for RGB8 {
//    fn from(color: RGB) -> Self {
//        Self {
//            r: (num::clamp(color.r, 0.0, 1.0) * 255f64) as u8,
//            g: (num::clamp(color.g, 0.0, 1.0) * 255f64) as u8,
//            b: (num::clamp(color.b, 0.0, 1.0) * 255f64) as u8,
//        }
//    }
//}
//
//impl Into<RGB> for RGB8 {
//    fn into(self) -> RGB {
//        RGB {
//            r: self.r as f64 / 255f64,
//            g: self.g as f64 / 255f64,
//            b: self.b as f64 / 255f64,
//        }
//    }
//}
