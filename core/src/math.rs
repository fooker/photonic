use num_traits::Float;
use palette::{Hsl, Hsv, Lch, Mix};
use palette::rgb::Rgb;
use crate::BufferReader;

pub trait Lerp {
    fn lerp(a: Self, b: Self, i: f32) -> Self;
}

impl Lerp for f64 {
    fn lerp(f1: Self, f2: Self, i: f32) -> Self {
        assert!((0.0..=1.0).contains(&i));

        if i <= 0.0 {
            return f1;
        }

        if i >= 1.0 {
            return f2;
        }

        return f1 + (f2 - f1) * i as f64;
    }
}

impl Lerp for f32 {
    fn lerp(f1: Self, f2: Self, i: f32) -> Self {
        assert!((0.0..=1.0).contains(&i));

        if i <= 0.0 {
            return f1;
        }

        if i >= 1.0 {
            return f2;
        }

        return f1 + (f2 - f1) * i;
    }
}

impl Lerp for Hsl {
    fn lerp(a: Self, b: Self, i: f32) -> Self {
        return Hsl::mix(a, b, i);
    }
}

impl Lerp for Hsv {
    fn lerp(a: Self, b: Self, i: f32) -> Self {
        return Hsv::mix(a, b, i);
    }
}

impl Lerp for Lch {
    fn lerp(a: Self, b: Self, i: f32) -> Self {
        return Lch::mix(a, b, i);
    }
}

impl Lerp for Rgb {
    fn lerp(a: Self, b: Self, i: f32) -> Self {
        return Rgb::mix(a, b, i);
    }
}

pub fn minmax<F>(a: F, b: F) -> (F, F)
    where F: PartialOrd,
{
    return if a < b { (a, b) } else { (b, a) };
}

pub fn clamp<F>(f: F, r: (F, F)) -> F
    where F: PartialOrd,
{
    if f <= r.0 {
        return r.0;
    }

    if f >= r.1 {
        return r.1;
    }

    return f;
}

pub fn clamp_opt<F>(f: F, r: (Option<F>, Option<F>)) -> F
    where F: PartialOrd,
{
    if let Some(r) = r.0 {
        if f <= r {
            return r;
        }
    }

    if let Some(r) = r.1 {
        if f >= r {
            return r;
        }
    }

    return f;
}

pub fn wrap<F>(f: F, s: usize) -> F
    where F: Float,
{
    let s = F::from(s).expect("Can not convert");
    return (s + (f % s)) % s;
}

pub fn remap<F>(v: F, s: (F, F), t: (F, F)) -> F
    where F: Float,
{
    return (v - s.0) / (s.1 - s.0) * (t.1 - t.0) + t.0;
}
