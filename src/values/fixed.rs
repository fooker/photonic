use core::{Dynamic,Value};
use std::time::Duration;

pub struct FixedValue(f64);

impl From<f64> for FixedValue {
    fn from(value: f64) -> Self {
        FixedValue(value)
    }
}

impl Value for FixedValue {
    fn name(&self) -> &str {
        ""
    }

    fn get(&self) -> f64 {
        self.0
    }
}

impl Dynamic for FixedValue {
    fn update(&mut self, duration: Duration) {}
}
