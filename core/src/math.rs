pub trait Lerp {
    fn lerp(a: Self, b: Self, i: f64) -> Self;
}

impl Lerp for f64 {
    fn lerp(f1: Self, f2: Self, i: f64) -> Self {
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

impl Lerp for f32 {
    fn lerp(f1: Self, f2: Self, i: f64) -> Self {
        assert!((0.0..=1.0).contains(&i));

        if i <= 0.0 {
            return f1;
        }

        if i >= 1.0 {
            return f2;
        }

        return f1 + (f2 - f1) * i as f32;
    }
}

pub fn minmax<F>(a: F, b: F) -> (F, F)
    where
        F: PartialOrd,
{
    return if a < b { (a, b) } else { (b, a) };
}

pub fn clamp<F>(f: F, r: (F, F)) -> F
    where
        F: PartialOrd,
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
    where
        F: PartialOrd,
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

pub fn wrap(f: f64, s: usize) -> f64 {
    let s = s as f64;
    return (s + (f % s)) % s;
}

pub fn remap(v: f64, s: (f64, f64), t: (f64, f64)) -> f64 {
    return (v - s.0) / (s.1 - s.0) * (t.1 - t.0) + t.0;
}
