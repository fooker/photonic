use std::borrow;
use std::time;

pub trait FractionalDuration {
    fn as_fractional_secs(&self) -> f64;
    fn as_fractional_millis(&self) -> f64;
    fn as_fractional_micros(&self) -> f64;
}

impl<T: borrow::Borrow<time::Duration>> FractionalDuration for T {
    fn as_fractional_secs(&self) -> f64 {
        let duration: &time::Duration = self.borrow();
        return duration.as_secs() as f64 * 1.0 + duration.subsec_nanos() as f64 / 1_000_000_000.0;
    }

    fn as_fractional_millis(&self) -> f64 {
        let duration: &time::Duration = self.borrow();
        return duration.as_secs() as f64 * 1_000.0 + duration.subsec_nanos() as f64 / 1_000_000.0;
    }

    fn as_fractional_micros(&self) -> f64 {
        let duration: &time::Duration = self.borrow();
        return duration.as_secs() as f64 * 1_000_000.0 + duration.subsec_nanos() as f64 / 1_000.0;
    }
}
