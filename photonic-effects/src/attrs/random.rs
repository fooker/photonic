use std::time::Duration;

use anyhow::Error;
use rand::{distributions::{Distribution, Uniform, uniform::SampleUniform}, rngs::SmallRng, SeedableRng};

use photonic_core::attr::{Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, Update};
use photonic_core::input::{Input, Poll};
use photonic_core::scene::{AttrBuilder, InputHandle};

pub struct Random<V>
where
    V: AttrValue + SampleUniform + Bounded,
{
    uniform: Uniform<V>,
    random: SmallRng,

    current: V,

    trigger: Input<()>,
}

impl<V> Attr<V> for Random<V>
where
    V: AttrValue + SampleUniform + Bounded,
{
    const KIND: &'static str = "random";

    fn get(&self) -> V {
        self.current
    }

    fn update(&mut self, _duration: Duration) -> Update<V> {
        if let Poll::Ready(()) = self.trigger.poll() {
            self.current = self.uniform.sample(&mut self.random);
            return Update::Changed(self.current);
        } else {
            return Update::Idle(self.current);
        }
    }
}

pub struct RandomDecl {
    pub trigger: InputHandle<()>,
}

impl<V> BoundAttrDecl<V> for RandomDecl
where
    V: AttrValue + SampleUniform + Bounded,
{
    type Target = Random<V>;
    fn materialize(
        self,
        bounds: Bounds<V>,
        builder: &mut AttrBuilder,
    ) -> Result<Self::Target, Error> {
        let mut random = SmallRng::from_entropy();
        let uniform = Uniform::new_inclusive(bounds.min, bounds.max);

        // Generate a random initial value
        let current = uniform.sample(&mut random);

        let trigger = builder.input("trigger", self.trigger)?;

        return Ok(Random {
            uniform,
            random,
            current,
            trigger,
        });
    }
}
