use crate::math;

pub type RGBColor = palette::LinSrgb<f64>;
pub type HSVColor = palette::Hsv<palette::encoding::Srgb, f64>;
pub type HSLColor = palette::Hsl<palette::encoding::Srgb, f64>;

use palette::Mix;

pub trait Black {
    fn black() -> Self;
}

impl math::Lerp for RGBColor {
    fn lerp(c1: Self, c2: Self, i: f64) -> Self {
        if i <= 0.0 {
            return c1;
        }

        if i >= 1.0 {
            return c2;
        }

        return Mix::mix(&c1, &c2, i);
    }
}

impl math::Lerp for HSVColor {
    fn lerp(c1: Self, c2: Self, i: f64) -> Self {
        if i <= 0.0 {
            return c1;
        }

        if i >= 1.0 {
            return c2;
        }

        return Mix::mix(&c1, &c2, i);
    }
}

impl math::Lerp for HSLColor {
    fn lerp(c1: Self, c2: Self, i: f64) -> Self {
        if i <= 0.0 {
            return c1;
        }

        if i >= 1.0 {
            return c2;
        }

        return Mix::mix(&c1, &c2, i);
    }
}

impl Black for RGBColor {
    fn black() -> Self {
        RGBColor::new(0.0, 0.0, 0.0)
    }
}

impl Black for HSVColor {
    fn black() -> Self {
        HSVColor::new(0.0, 0.0, 0.0)
    }
}

impl Black for HSLColor {
    fn black() -> Self {
        HSLColor::new(0.0, 0.0, 0.0)
    }
}
