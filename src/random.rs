use palette::Mix;
use std::time::Duration;

use rand::distr::uniform::SampleUniform;
use rand::distr::{Distribution, StandardUniform};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::math;
use crate::math::Lerp;

#[derive(Debug)]
pub struct Random(SmallRng);

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Random {
    pub fn new() -> Self {
        Self(SmallRng::from_os_rng())
    }

    pub fn rate(&mut self, value: f64, duration: Duration) -> bool {
        let chance = math::clamp(duration.as_secs_f64() * value, (0.0, 1.0));
        return self.0.random_bool(chance);
    }

    pub fn lerp<V>(&mut self, v1: V, v2: V) -> V
    where V: Lerp {
        let v = self.0.random();
        return Lerp::lerp(v1, v2, v);
    }

    // TODO: unify mix with lerp
    pub fn mix<V>(&mut self, v1: V, v2: V) -> V
    where
        V: Mix,
        StandardUniform: Distribution<<V as Mix>::Scalar>,
    {
        let v = self.0.random();
        return Mix::mix(v1, v2, v);
    }

    #[allow(clippy::float_cmp)]
    pub fn range<F: PartialOrd + SampleUniform>(&mut self, min: F, max: F) -> F {
        let values = math::minmax(min, max);
        if values.0 == values.1 {
            return values.0;
        }

        return self.0.random_range(values.0..=values.1);
    }
}
