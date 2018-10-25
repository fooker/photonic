pub trait Lerp {
    fn lerp(a: Self, b: Self, v: f64) -> Self;
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

pub fn wrap(f: f64, s: usize) -> f64 {
    let f = f % (s as f64);
    if f.is_sign_positive() {
        return f;
    } else {
        return f + (s as f64);
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wrap() {
        assert_approx_eq!(wrap(0.0, 10), 0.0);
        assert_approx_eq!(wrap(1.0, 10), 1.0);
        assert_approx_eq!(wrap(1.1, 10), 1.1);


        assert_approx_eq!(wrap(12.34, 10), 2.34);

        assert_approx_eq!(wrap(-5.6, 10), 4.4);
        assert_approx_eq!(wrap(-23.4, 10), 6.6);
    }
}
