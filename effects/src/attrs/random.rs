use std::marker::PhantomData;
use std::time::Duration;

use anyhow::Error;
use rand::distributions::{Distribution, Uniform};
use rand::distributions::uniform::SampleUniform;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use photonic::{Attr, AttrBuilder, AttrValue, BoundAttrDecl};
use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll};
use photonic::scene::InputHandle;

pub struct RandomAttr<V>
    where V: AttrValue + SampleUniform + Bounded,
{
    uniform: Uniform<V>,
    random: SmallRng,

    current: V,

    trigger: Input<()>,
}

impl<V> Attr for RandomAttr<V>
    where V: AttrValue + SampleUniform + Bounded,
{
    type Value = V;
    const KIND: &'static str = "random";

    fn update(&mut self, _duration: Duration) -> V {
        if let Poll::Update(()) = self.trigger.poll() {
            self.current = self.uniform.sample(&mut self.random);
        }

        return self.current;
    }
}

pub struct Random<V> {
    pub trigger: InputHandle<()>,

    phantom: PhantomData<V>,
}

impl<V> BoundAttrDecl for Random<V>
    where V: AttrValue + SampleUniform + Bounded,
{
    type Value = V;
    type Attr = RandomAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr, Error> {
        let mut random = SmallRng::from_entropy();
        let uniform = Uniform::new_inclusive(bounds.min, bounds.max);

        // Generate a random initial value
        let current = uniform.sample(&mut random);

        let trigger = builder.input("trigger", self.trigger)?;

        return Ok(RandomAttr {
            uniform,
            random,
            current,
            trigger,
        });
    }
}
