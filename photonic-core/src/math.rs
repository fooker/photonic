pub trait Lerp {
    fn lerp(a: Self, b: Self, i: f64) -> Self;
}

impl Lerp for f64 {
    fn lerp(f1: Self, f2: Self, i: f64) -> Self {
        assert!(0.0 <= i && i <= 1.0);

        if i == 0.0 {
            return f1;
        }

        if i == 1.0 {
            return f2;
        }

        return f1 + (f2 - f1) * i;
    }
}

impl Lerp for f32 {
    fn lerp(f1: Self, f2: Self, i: f64) -> Self {
        assert!(0.0 <= i && i <= 1.0);

        if i == 0.0 {
            return f1;
        }

        if i == 1.0 {
            return f2;
        }

        return f1 + (f2 - f1) * i as f32;
    }
}

pub fn minmax<F>(a: F, b: F) -> (F, F)
    where F: PartialOrd {
    if a < b {
        return (a, b);
    } else {
        return (b, a);
    }
}

pub fn clamp<F>(f: F, r: (F, F)) -> F
    where F: PartialOrd {
    if f <= r.0 {
        return r.0;
    }

    if f >= r.1 {
        return r.1;
    }

    return f;
}

pub fn clamp_opt<F>(f: F, r: (Option<F>, Option<F>)) -> F
    where F: PartialOrd {
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


#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_wrap() {
        assert_approx_eq!(wrap(0.0, 13), 0.0);
        assert_approx_eq!(wrap(0.2, 13), 0.2);
        assert_approx_eq!(wrap(1.0, 13), 1.0);
        assert_approx_eq!(wrap(12.9, 13), 12.9);

        assert_approx_eq!(wrap(13.0, 13), 0.0);
        assert_approx_eq!(wrap(13.5, 13), 0.5);
        assert_approx_eq!(wrap(15.0, 13), 2.0);

        assert_approx_eq!(wrap(-0.0, 5), 0.0);
        assert_approx_eq!(wrap(-1.0, 5), 4.0);
        assert_approx_eq!(wrap(-2.0, 5), 3.0);
        assert_approx_eq!(wrap(-3.0, 5), 2.0);
        assert_approx_eq!(wrap(-4.0, 5), 1.0);
        assert_approx_eq!(wrap(-5.0, 5), 0.0);
        assert_approx_eq!(wrap(-6.0, 5), 4.0);
        assert_approx_eq!(wrap(-7.0, 5), 3.0);
        assert_approx_eq!(wrap(-8.0, 5), 2.0);
        assert_approx_eq!(wrap(-9.0, 5), 1.0);

        assert_approx_eq!(wrap(-0.3, 13), 12.7);
        assert_approx_eq!(wrap(-1.0, 13), 12.0);
        assert_approx_eq!(wrap(-1.6, 13), 11.4);
        assert_approx_eq!(wrap(-5.0, 13), 8.0);
        assert_approx_eq!(wrap(-11.0, 13), 2.0);
        assert_approx_eq!(wrap(-12.0, 13), 1.0);
        assert_approx_eq!(wrap(-12.2, 13), 0.8);
        assert_approx_eq!(wrap(-13.0, 13), 0.0);
        assert_approx_eq!(wrap(-13.6, 13), 12.4);
        assert_approx_eq!(wrap(-14.0, 13), 12.0);
        assert_approx_eq!(wrap(-23.0, 13), 3.0);
    }

    #[test]
    fn test_remap() {
        assert_approx_eq!(remap(0.0, (0.0, 1.0), (0.0, 1.0)), 0.0);
        assert_approx_eq!(remap(1.0, (0.0, 1.0), (0.0, 1.0)), 1.0);
        assert_approx_eq!(remap(0.5, (0.0, 1.0), (0.0, 1.0)), 0.5);

        assert_approx_eq!(remap(0.0, (0.0, 0.1), (0.0, 1.0)), 0.0);
        assert_approx_eq!(remap(0.1, (0.0, 0.1), (0.0, 1.0)), 1.0);
        assert_approx_eq!(remap(1.0, (0.0, 0.1), (0.0, 1.0)), 10.0);

        assert_approx_eq!(remap( 0.0, (-1.0, 1.0), (0.0, 10.0)), 5.0);
        assert_approx_eq!(remap( 1.0, (-1.0, 1.0), (0.0, 10.0)), 10.0);
        assert_approx_eq!(remap(-1.0, (-1.0, 1.0), (0.0, 10.0)), 0.0);
    }
}
