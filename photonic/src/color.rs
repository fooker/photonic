use crate::math;

pub use scarlet::color::Color;
pub use scarlet::color::RGBColor;
pub use scarlet::colors::HSVColor;
pub use scarlet::colors::HSLColor;

pub trait Black {
    fn black() -> Self;
}

impl math::Lerp for RGBColor {
    fn lerp(c1: Self, c2: Self, f: f64) -> Self {
        use scarlet::colorpoint::ColorPoint;

        if f <= 0.0 {
            return c1;
        }

        if f >= 1.0 {
            return c2;
        }

        return ColorPoint::weighted_midpoint(c2, c1, f);
    }
}

impl Black for RGBColor {
    fn black() -> Self {
        RGBColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Black for HSVColor {
    fn black() -> Self {
        HSVColor {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        }
    }
}

impl Black for HSLColor {
    fn black() -> Self {
        HSLColor {
            h: 0.0,
            s: 0.0,
            l: 0.0,
        }
    }
}
