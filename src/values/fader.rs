use core::{Dynamic,Value};
use std::time::Duration;

pub struct FaderValue {
    name: String,
    value: f64,
}

impl Value for FaderValue {
    fn name(&self) -> &str {
        &self.name
    }

    fn get(&self) -> f64 {
        self.value
    }
}

impl Dynamic for FaderValue {
    fn update(&mut self, duration: Duration) {}
}
