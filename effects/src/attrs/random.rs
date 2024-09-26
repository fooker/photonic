use std::marker::PhantomData;

use anyhow::Result;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use photonic::attr::{Bounded, Bounds};
use photonic::input::{Input, Poll};
use photonic::scene::InputHandle;
use photonic::{scene, Attr, AttrBuilder, AttrValue, BoundAttrDecl};

pub struct RandomAttr<V>
where V: AttrValue + SampleUniform + Bounded
{
    uniform: Uniform<V>,
    random: SmallRng,

    current: V,

    trigger: Input<()>,
}

impl<V> Attr for RandomAttr<V>
where V: AttrValue + SampleUniform + Bounded
{
    type Value = V;
    const KIND: &'static str = "random";

    fn update(&mut self, _ctx: &scene::RenderContext) -> Self::Value {
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
where V: AttrValue + SampleUniform + Bounded
{
    type Value = V;
    type Attr = RandomAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Attr> {
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

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use photonic_dynamic::factory::Producible;
    use photonic_dynamic::registry::Registry;
    use photonic_dynamic::{builder, config, DynBoundAttrDecl};

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub trigger: config::Input,
    }

    impl<V> Producible<dyn DynBoundAttrDecl<V>> for Config
    where V: AttrValue + DeserializeOwned + Bounded + SampleUniform
    {
        type Product = Random<V>;

        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> Result<Self::Product> {
            return Ok(Random {
                trigger: builder.input(config.trigger)?,
                phantom: Default::default(),
            });
        }
    }
}
