use std::time::Duration;

use num_traits::Float;

pub struct Easing<F: Float> {
    pub func: fn(F) -> F,
    pub speed: Duration,
}

impl<F: Float> Easing<F> {
    pub fn new(func: fn(F) -> F) -> Self {
        return Self {
            func,
            speed: Duration::from_secs(1),
        };
    }

    pub fn with_speed(mut self, speed: Duration) -> Self {
        self.speed = speed;
        return self;
    }
}

impl<F: Float> From<fn(F) -> F> for Easing<F> {
    fn from(func: fn(F) -> F) -> Self {
        return Self::new(func);
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use super::*;

    use photonic_dynamic::serde::{Deserialize, Deserializer};

    impl<'de, F: Float> Deserialize<'de> for Easing<F> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
            #[derive(Debug, Deserialize)]
            struct S {
                func: Easings,
                speed: Duration,
            }

            let s = S::deserialize(deserializer)?;

            return Ok(s.func.with_speed(s.speed));
        }
    }

    #[derive(Debug, Deserialize)]
    pub enum EasingDirection {
        In,
        Out,
        InOut,
    }

    #[derive(Debug, Deserialize)]
    pub enum Easings {
        Linear,
        Quadratic(EasingDirection),
        Cubic(EasingDirection),
        Quartic(EasingDirection),
        Quintic(EasingDirection),
        Sine(EasingDirection),
        Circular(EasingDirection),
        Exponential(EasingDirection),
        Elastic(EasingDirection),
        Back(EasingDirection),
        Bounce(EasingDirection),
    }

    impl Easings {
        pub fn with_speed<F: Float>(self, speed: Duration) -> Easing<F> {
            return Easing::from(self).with_speed(speed);
        }
    }

    impl<F: Float> From<Easings> for Easing<F> {
        fn from(value: Easings) -> Self {
            use ezing::*;
            return match value {
                Easings::Linear => linear,
                Easings::Quadratic(EasingDirection::In) => quad_in,
                Easings::Quadratic(EasingDirection::Out) => quad_out,
                Easings::Quadratic(EasingDirection::InOut) => quad_inout,
                Easings::Cubic(EasingDirection::In) => cubic_in,
                Easings::Cubic(EasingDirection::Out) => cubic_out,
                Easings::Cubic(EasingDirection::InOut) => cubic_inout,
                Easings::Quartic(EasingDirection::In) => quart_in,
                Easings::Quartic(EasingDirection::Out) => quart_out,
                Easings::Quartic(EasingDirection::InOut) => quad_inout,
                Easings::Quintic(EasingDirection::In) => quint_in,
                Easings::Quintic(EasingDirection::Out) => quint_out,
                Easings::Quintic(EasingDirection::InOut) => quint_inout,
                Easings::Sine(EasingDirection::In) => sine_in,
                Easings::Sine(EasingDirection::Out) => sine_out,
                Easings::Sine(EasingDirection::InOut) => sine_inout,
                Easings::Circular(EasingDirection::In) => circ_in,
                Easings::Circular(EasingDirection::Out) => circ_out,
                Easings::Circular(EasingDirection::InOut) => circ_inout,
                Easings::Exponential(EasingDirection::In) => expo_in,
                Easings::Exponential(EasingDirection::Out) => expo_out,
                Easings::Exponential(EasingDirection::InOut) => expo_inout,
                Easings::Elastic(EasingDirection::In) => elastic_in,
                Easings::Elastic(EasingDirection::Out) => elastic_out,
                Easings::Elastic(EasingDirection::InOut) => elastic_inout,
                Easings::Back(EasingDirection::In) => back_in,
                Easings::Back(EasingDirection::Out) => back_out,
                Easings::Back(EasingDirection::InOut) => back_inout,
                Easings::Bounce(EasingDirection::In) => bounce_in,
                Easings::Bounce(EasingDirection::Out) => bounce_out,
                Easings::Bounce(EasingDirection::InOut) => bounce_inout,
            }
            .into();
        }
    }
}
