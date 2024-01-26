use std::time::Duration;
use palette::Mix;

use rand::{Rng, SeedableRng};
use rand::distributions::Distribution;
use rand::distributions::uniform::SampleUniform;
use rand::rngs::SmallRng;
use rand::distributions::Standard;

use crate::math;
use crate::math::Lerp;

pub struct Random(SmallRng);

impl Random {
    pub fn new() -> Self {
        Self(SmallRng::from_entropy())
    }

    pub fn rate(&mut self, value: f64, duration: Duration) -> bool {
        let chance = math::clamp(duration.as_secs_f64() * value, (0.0, 1.0));
        return self.0.gen_bool(chance);
    }

    pub fn lerp<V>(&mut self, v1: V, v2: V) -> V
        where V: Lerp,
    {
        let v = self.0.gen();
        return Lerp::lerp(v1, v2, v);
    }

    // TODO: unify mix with lerp
    pub fn mix<V>(&mut self, v1: V, v2: V) -> V
        where V: Mix,
              Standard: Distribution<<V as Mix>::Scalar>,
    {
        let v = self.0.gen();
        return Mix::mix(v1, v2, v);
    }

    #[allow(clippy::float_cmp)]
    pub fn range<F: PartialOrd + SampleUniform>(&mut self, min: F, max: F) -> F {
        let values = math::minmax(min, max);
        if values.0 == values.1 {
            return values.0;
        }

        return self.0.gen_range(values.0..=values.1);
    }
}